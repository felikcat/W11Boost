use eframe::egui::{self, Color32, RichText, ScrollArea};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use std::sync::{Arc, Mutex};

use super::config::{TweakConfig, get_config_dir};
use super::shared_state::{SharedState, WorkerContext};
use super::state::{NavigationEntry, SelectionState, TweakStates, ViewMode};
use super::theme;
use super::tweaks::{CATEGORIES, Tweak, apply_tweak, get_all_tweaks, get_tweaks_for_category};
use super::widgets;

#[cfg(windows)]
fn enforce_dark_mode(window: &eframe::Frame)
{
        if let Ok(handle) = window.window_handle() {
                if let RawWindowHandle::Win32(h) = handle.as_raw() {
                        // h.hwnd is NonZeroIsize, so .get() returns isize
                        let _ = crate::common::apply_dark_mode_to_window(h.hwnd.get());
                }
        }
}

/// Main application
pub struct W11BoostApp
{
        pub mode: ViewMode,
        pub selection: SelectionState,
        pub tweak_states: TweakStates,
        pub show_log_panel: bool,
        pub search_query: String,
        pub shared: Arc<Mutex<SharedState>>,
        pub dark_mode_enforced: bool,
        // State for searching within custom input boxes
        pub input_search_query: String,
        pub input_search_visible: bool,
        // Navigation history
        pub back_stack: Vec<NavigationEntry>,
        pub forward_stack: Vec<NavigationEntry>,
}

impl W11BoostApp
{
        pub fn new(cc: &eframe::CreationContext<'_>) -> Self
        {
                theme::apply_dark_theme(&cc.egui_ctx);

                // Expand all categories by default
                let mut expanded = std::collections::HashMap::new();
                for cat in CATEGORIES {
                        expanded.insert(cat.id.to_string(), true);
                }

                let mut tweak_states = TweakStates::default();

                // Try to load dynamic config if it exists
                if let Ok(config) = TweakConfig::load_default() {
                        for (id, enabled) in config.tweaks {
                                tweak_states.states.insert(id, enabled);
                        }
                        tweak_states.input_values = config.input_values;
                }

                Self {
                        mode: ViewMode::Tweaks,
                        selection: SelectionState {
                                selected_category: None,
                                selected_tweak: None,
                                expanded_categories: expanded,
                        },
                        tweak_states,
                        show_log_panel: false,
                        search_query: String::new(),
                        shared: Arc::new(Mutex::new(SharedState::default())),
                        dark_mode_enforced: false,
                        input_search_query: String::new(),
                        input_search_visible: false,
                        back_stack: Vec::new(),
                        forward_stack: Vec::new(),
                }
        }

        fn autosave(&self)
        {
                let config = TweakConfig::new(self.tweak_states.states.clone(), self.tweak_states.input_values.clone());
                let _ = config.save_default();
        }

        fn count_enabled_tweaks(&self) -> usize
        {
                get_all_tweaks()
                        .iter()
                        .filter(|t| self.tweak_states.states.get(t.id).copied().unwrap_or(false))
                        .count()
        }

        fn get_enabled_tweaks(&self) -> Vec<&'static Tweak>
        {
                get_all_tweaks()
                        .into_iter()
                        .filter(|t| self.tweak_states.states.get(t.id).copied().unwrap_or(false))
                        .collect()
        }

        fn is_running(&self) -> bool
        {
                let state = self.shared.lock().unwrap();
                state.is_running
        }

        fn spawn_tweaks_worker(&self, ctx: egui::Context, skip_restore_point: bool)
        {
                let enabled_tweaks = self.get_enabled_tweaks();
                let total_ops: u32 = enabled_tweaks.iter().map(|t| t.op_count()).sum();
                let shared = Arc::clone(&self.shared);

                // Collect tweak IDs to apply
                let tweak_ids: Vec<&'static str> = enabled_tweaks.iter().map(|t| t.id).collect();
                let input_values = self.tweak_states.input_values.clone();

                use rayon::prelude::*;

                std::thread::spawn(move || {
                        let worker_ctx = WorkerContext::new(shared, ctx, total_ops, input_values);

                        if !skip_restore_point {
                                worker_ctx.post_status("Creating System Restore Point...");

                                // Logic to create a system restore point
                                // 1. Force SystemRestorePointCreationFrequency to 0 to bypass the 24h limit.
                                // 2. Create the restore point.
                                // 3. Revert SystemRestorePointCreationFrequency if the user did NOT enable the tweak to remove the limit.

                                let restore_point_tweak_id = "restore_point_frequency";
                                let restore_point_limit_removed = tweak_ids.contains(&restore_point_tweak_id);

                                // Ensure we can create a restore point by setting frequency to 0
                                let _ = crate::common::run_system_command(
                                        "reg",
                                        &[
                                                "add",
                                                r"HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\SystemRestore",
                                                "/v",
                                                "SystemRestorePointCreationFrequency",
                                                "/t",
                                                "REG_DWORD",
                                                "/d",
                                                "0",
                                                "/f",
                                        ],
                                );

                                // Force enable System Protection for C: drive
                                if let Err(e) =
                                        crate::common::run_powershell_command("Enable-ComputerRestore -Drive 'C:\\'")
                                {
                                        worker_ctx.post_error(format!("Failed to enable System Protection: {}", e));
                                }

                                // Attempt to create the restore point
                                // We use Checkpoint-Computer which is available on Win10/11
                                if let Err(e) = crate::common::run_powershell_command(
                                        "Checkpoint-Computer -Description 'W11Boost Auto-Restore' -RestorePointType 'MODIFY_SETTINGS' -ErrorAction SilentlyContinue",
                                ) {
                                        worker_ctx.post_error(format!("Failed to create restore point: {}", e));
                                }

                                // Revert the registry change if the user didn't ask for it
                                if !restore_point_limit_removed {
                                        // Default behavior is usually that the key doesn't exist or is set to default (we delete it to be safe/clean)
                                        let _ = crate::common::run_system_command(
                                                "reg",
                                                &[
                                                        "delete",
                                                        r"HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\SystemRestore",
                                                        "/v",
                                                        "SystemRestorePointCreationFrequency",
                                                        "/f",
                                                ],
                                        );
                                }
                        } else {
                                worker_ctx.post_status("Skipping Restore Point...");
                        }

                        tweak_ids.into_par_iter().for_each(|tweak_id| {
                                // Find the tweak again in the static data
                                if let Some(tweak) = get_all_tweaks().into_iter().find(|t| t.id == tweak_id) {
                                        if let Err(e) = apply_tweak(tweak, &worker_ctx) {
                                                worker_ctx.post_error(format!("Failed to apply {}: {}", tweak.name, e));
                                        }
                                }
                        });

                        worker_ctx.post_complete();
                });
        }

        fn save_config(&self)
        {
                let default_dir = get_config_dir();

                let mut dialog = rfd::FileDialog::new()
                        .add_filter("JSON", &["json"])
                        .set_file_name("config.json");

                if let Some(dir) = default_dir {
                        let _ = std::fs::create_dir_all(&dir);
                        dialog = dialog.set_directory(&dir);
                }

                if let Some(path) = dialog.save_file() {
                        let config = TweakConfig::new(
                                self.tweak_states.states.clone(),
                                self.tweak_states.input_values.clone(),
                        );
                        if let Err(e) = config.save_to_file(&path) {
                                let mut state = self.shared.lock().unwrap();
                                state.status_message = format!("Failed to save: {e}");
                        } else {
                                let mut state = self.shared.lock().unwrap();
                                state.status_message = format!("Saved to {}", path.display());
                        }
                }
        }

        fn load_config(&mut self)
        {
                let default_dir = get_config_dir();

                let mut dialog = rfd::FileDialog::new().add_filter("JSON", &["json"]);

                if let Some(dir) = default_dir
                        && dir.exists()
                {
                        dialog = dialog.set_directory(&dir);
                }

                if let Some(path) = dialog.pick_file() {
                        match TweakConfig::load_from_file(&path) {
                                Ok(config) => {
                                        // Merge loaded config with existing states
                                        for (id, enabled) in config.tweaks {
                                                self.tweak_states.states.insert(id, enabled);
                                        }
                                        self.tweak_states.input_values.extend(config.input_values);
                                        self.autosave();
                                        let mut state = self.shared.lock().unwrap();
                                        state.status_message = format!("Loaded from {}", path.display());
                                }
                                Err(e) => {
                                        let mut state = self.shared.lock().unwrap();
                                        state.status_message = format!("Failed to load: {e}");
                                }
                        }
                }
        }

        fn current_nav_state(&self) -> NavigationEntry
        {
                NavigationEntry {
                        mode: self.mode,
                        selected_category: self.selection.selected_category.clone(),
                        selected_tweak: self.selection.selected_tweak.clone(),
                        search_query: self.search_query.clone(),
                }
        }

        fn apply_nav_state(&mut self, state: NavigationEntry)
        {
                self.mode = state.mode;
                self.selection.selected_category = state.selected_category;
                self.selection.selected_tweak = state.selected_tweak;
                self.search_query = state.search_query;
        }

        fn navigate_to(&mut self, mode: ViewMode, category: Option<String>, tweak: Option<String>)
        {
                // Capture current state
                let current = self.current_nav_state();

                // Only push to history if new state is different
                let new_state = NavigationEntry {
                        mode,
                        selected_category: category.clone(),
                        selected_tweak: tweak.clone(),
                        search_query: self.search_query.clone(), // Preserve search query by default
                };

                if current != new_state {
                        self.back_stack.push(current);
                        self.forward_stack.clear();
                        self.apply_nav_state(new_state);
                }
        }

        fn navigate_back(&mut self)
        {
                if let Some(prev) = self.back_stack.pop() {
                        let current = self.current_nav_state();
                        self.forward_stack.push(current);
                        self.apply_nav_state(prev);
                }
        }

        fn navigate_forward(&mut self)
        {
                if let Some(next) = self.forward_stack.pop() {
                        let current = self.current_nav_state();
                        self.back_stack.push(current);
                        self.apply_nav_state(next);
                }
        }

        /// Render the left sidebar with category tree

        /// Render the left sidebar with category tree
        pub fn render_sidebar(&mut self, ui: &mut egui::Ui)
        {
                if self.is_running() {
                        ui.disable();
                }

                ui.add_space(8.0);
                self.render_search_box(ui);
                ui.add_space(8.0);

                egui::Frame::NONE
                        .inner_margin(egui::Margin {
                                left: 0,
                                right: 16,
                                top: 0,
                                bottom: 0,
                        })
                        .show(ui, |ui| {
                                if ui.add(egui::Button::new(RichText::new("Main Screen").size(14.0))
                                        .min_size(egui::vec2(ui.available_width(), 32.0))
                                        .fill(ui.style().visuals.faint_bg_color))
                                        .clicked()
                                {
                                        self.navigate_to(ViewMode::Tweaks, None, None);
                                }
                        });

                ui.add_space(12.0);
                self.render_category_list(ui);
        }

        fn render_search_box(&mut self, ui: &mut egui::Ui)
        {
                egui::Frame::NONE
                        .inner_margin(egui::Margin {
                                left: 0,
                                right: 16,
                                top: 0,
                                bottom: 0,
                        })
                        .show(ui, |ui| {
                                ui.add(egui::TextEdit::singleline(&mut self.search_query)
                                        .hint_text("Search tweaks...")
                                        .desired_width(ui.available_width())
                                        .margin(egui::Margin::same(8)));
                        });
        }

        fn render_category_list(&mut self, ui: &mut egui::Ui)
        {
                ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        egui::Frame::NONE
                                .inner_margin(egui::Margin {
                                        left: 0,
                                        right: 16,
                                        top: 0,
                                        bottom: 0,
                                })
                                .show(ui, |ui| {
                                        for category in CATEGORIES {
                                                self.render_category_node(ui, category);
                                        }
                                });
                });
        }

        fn get_visible_tweaks(&self, tweaks: Vec<&'static Tweak>) -> Vec<&'static Tweak>
        {
                if self.search_query.is_empty() {
                        return tweaks;
                }

                let query = self.search_query.to_lowercase();
                tweaks.into_iter()
                        .filter(|t| {
                                t.name.to_lowercase().contains(&query) || t.description.to_lowercase().contains(&query)
                        })
                        .collect()
        }

        fn render_category_node(&mut self, ui: &mut egui::Ui, category: &super::tweaks::TweakCategory)
        {
                let tweaks = get_tweaks_for_category(category.id);
                let visible_tweaks = self.get_visible_tweaks(tweaks);

                if visible_tweaks.is_empty() && !self.search_query.is_empty() {
                        return;
                }

                let is_expanded = self
                        .selection
                        .expanded_categories
                        .get(category.id)
                        .copied()
                        .unwrap_or(true);

                let is_selected = self
                        .selection
                        .selected_category
                        .as_ref()
                        .is_some_and(|s| s == category.id)
                        && self.selection.selected_tweak.is_none();

                // Allocate space for the row
                let (rect, response) =
                        ui.allocate_at_least(egui::vec2(ui.available_width(), 32.0), egui::Sense::click());

                // Interaction state
                let is_hovered = ui.rect_contains_pointer(rect);

                // Draw background
                let bg_color = if is_selected {
                        ui.style().visuals.selection.bg_fill
                } else if is_hovered {
                        ui.style().visuals.widgets.hovered.bg_fill
                } else {
                        Color32::TRANSPARENT
                };

                if bg_color != Color32::TRANSPARENT {
                        ui.painter()
                                .rect_filled(rect, ui.style().visuals.window_corner_radius, bg_color);
                }

                // Selection indicator (vertical bar)
                if is_selected {
                        let mut indicator_rect = rect;
                        indicator_rect.max.x = indicator_rect.min.x + 3.0;
                        indicator_rect = indicator_rect.shrink2(egui::vec2(0.0, 8.0));
                        ui.painter().rect_filled(
                                indicator_rect,
                                ui.style().visuals.window_corner_radius,
                                ui.style().visuals.selection.stroke.color,
                        );
                }

                // Draw content
                let text_color = ui.style().visuals.widgets.noninteractive.fg_stroke.color;

                ui.painter().text(
                        rect.min + egui::vec2(12.0, rect.height() / 2.0),
                        egui::Align2::LEFT_CENTER,
                        &category.name,
                        egui::FontId::proportional(14.0),
                        text_color,
                );

                if response.clicked() {
                        self.navigate_to(ViewMode::Tweaks, Some(category.id.to_string()), None);

                        // Toggle expansion on click
                        self.selection
                                .expanded_categories
                                .insert(category.id.to_string(), !is_expanded);
                }

                if is_expanded {
                        self.render_category_tweaks(ui, category.id, visible_tweaks);
                }
                ui.add_space(2.0);
        }

        fn render_category_tweaks(&mut self, ui: &mut egui::Ui, category_id: &str, tweaks: Vec<&'static Tweak>)
        {
                ui.indent(category_id, |ui| {
                        for tweak in tweaks {
                                let is_tweak_selected =
                                        self.selection.selected_tweak.as_ref().is_some_and(|s| s == tweak.id);
                                let is_enabled = self.tweak_states.states.get(tweak.id).copied().unwrap_or(false);

                                let text_color = if is_enabled {
                                        ui.style().visuals.selection.stroke.color
                                } else {
                                        ui.style().visuals.widgets.noninteractive.fg_stroke.color
                                };

                                let response = ui.selectable_label(
                                        is_tweak_selected,
                                        RichText::new(format!("  {}", tweak.name)).color(text_color),
                                );

                                if response.clicked() {
                                        self.navigate_to(
                                                ViewMode::Tweaks,
                                                Some(category_id.to_string()),
                                                Some(tweak.id.to_string()),
                                        );
                                }
                        }
                });
        }

        /// Render the main content panel
        pub fn render_content(&mut self, ui: &mut egui::Ui)
        {
                match self.mode {
                        ViewMode::Tweaks => {
                                if let Some(tweak_id) = &self.selection.selected_tweak.clone() {
                                        self.render_tweak_detail(ui, tweak_id);
                                } else if let Some(cat_id) = &self.selection.selected_category.clone() {
                                        self.render_category_view(ui, cat_id);
                                } else {
                                        self.render_welcome(ui);
                                }
                        }
                        ViewMode::ConfirmApply => {
                                self.render_confirm_apply(ui);
                        }
                        ViewMode::ConfirmRestorePoint => {
                                self.render_confirm_restore_point(ui);
                        }
                        ViewMode::ConfirmUnsetAll => {
                                self.render_confirm_unset_all(ui);
                        }
                        ViewMode::SelectedTweaks => {
                                self.render_selected_tweaks(ui);
                        }
                        ViewMode::ConfirmLoadDefaults => {
                                self.render_confirm_load_defaults(ui);
                        }
                }
        }

        fn render_selected_tweaks(&mut self, ui: &mut egui::Ui)
        {
                if ui.button(RichText::new("\u{2190}  Back to Tweaks").strong()).clicked() {
                        self.navigate_back();
                }
                ui.add_space(20.0);

                ui.heading(RichText::new("Currently Selected Tweaks").size(28.0));
                ui.add_space(12.0);

                ui.vertical(|ui| {
                        ui.set_max_width(800.0);
                        let enabled_tweaks = self.get_enabled_tweaks();
                        let visible_tweaks = self.get_visible_tweaks(enabled_tweaks);

                        if visible_tweaks.is_empty() {
                                if self.search_query.is_empty() {
                                        ui.label("No tweaks selected.");
                                } else {
                                        ui.label(format!("No selected tweaks match '{}'", self.search_query));
                                }
                        } else {
                                if self.search_query.is_empty() {
                                        ui.label(format!("Showing {} selected tweaks:", visible_tweaks.len()));
                                } else {
                                        ui.label(format!(
                                                "Showing {} matches for '{}' in selected tweaks:",
                                                visible_tweaks.len(),
                                                self.search_query
                                        ));
                                }
                                ui.add_space(12.0);

                                for tweak in visible_tweaks {
                                        self.render_tweak_row(ui, tweak);
                                        ui.add_space(4.0);
                                }
                        }
                });
        }

        fn render_welcome_category_button(&mut self, ui: &mut egui::Ui, category: &super::tweaks::TweakCategory)
        {
                let button = egui::Button::new(RichText::new(category.name).size(16.0))
                        .min_size(egui::vec2(180.0, 60.0))
                        .fill(ui.style().visuals.faint_bg_color)
                        .stroke(egui::Stroke::new(
                                1.0,
                                ui.style().visuals.widgets.inactive.bg_stroke.color,
                        ));

                if ui.add(button).on_hover_text(category.description).clicked() {
                        self.navigate_to(ViewMode::Tweaks, Some(category.id.to_string()), None);
                }
        }

        fn render_welcome(&mut self, ui: &mut egui::Ui)
        {
                ui.add_space(40.0);
                ui.vertical_centered(|ui| {
                        ui.heading(RichText::new("Welcome to W11Boost").size(32.0).strong());
                        ui.add_space(12.0);
                        ui.label(RichText::new("A sizable customization suite.").size(16.0).weak());
                        ui.add_space(20.0);
                        
                        ui.group(|ui| {
                            ui.set_max_width(600.0);
                            ui.vertical_centered(|ui| {
                                ui.label(RichText::new("⚠️SAFETY WARNING").color(Color32::from_rgb(255, 165, 0)).strong());
                                ui.label("Do not trust other people's configurations without looking through them first.");
                            });
                        });

                        ui.add_space(32.0);

                        ui.vertical_centered(|ui| {
                                ui.set_max_width(800.0);
                                ui.horizontal_wrapped(|ui| {
                                        ui.spacing_mut().item_spacing = egui::vec2(12.0, 12.0);
                                        for category in CATEGORIES {
                                                self.render_welcome_category_button(ui, category);
                                        }
                                });
                        });
                });
        }

        fn render_tweak_tree_node(&mut self, ui: &mut egui::Ui, tweak: &'static Tweak)
        {
                if !tweak.sub_tweaks.is_empty() {
                        let id_source = format!("collapse_{}", tweak.id);
                        let count = tweak.sub_tweaks.len();
                        ui.push_id(id_source, |ui| {
                                egui::CollapsingHeader::new(
                                        RichText::new(format!("{} ({} items)", tweak.name, count))
                                                .strong()
                                                .size(16.0),
                                )
                                .default_open(false)
                                .show(ui, |ui| {
                                        ui.add_space(4.0);
                                        // Render parent
                                        self.render_tweak_row(ui, tweak);

                                        ui.indent("sub_indent", |ui| {
                                                for sub in tweak.sub_tweaks {
                                                        self.render_tweak_row(ui, sub);
                                                        ui.add_space(4.0);
                                                }
                                        });
                                });
                        });
                        ui.add_space(4.0);
                } else {
                        self.render_tweak_row(ui, tweak);
                        ui.add_space(4.0);
                }
        }

        fn render_tweak_row_inner(&mut self, ui: &mut egui::Ui, tweak: &'static Tweak, is_enabled: bool)
        {
                ui.set_width(ui.available_width());
                ui.horizontal(|ui| {
                        let mut enabled = is_enabled;
                        ui.scope(|ui| {
                                ui.style_mut().spacing.icon_width = 28.0;
                                ui.style_mut().spacing.icon_spacing = 10.0;
                                if ui.checkbox(&mut enabled, "").changed() {
                                        self.tweak_states.states.insert(tweak.id.to_string(), enabled);
                                        self.autosave();
                                }
                        });

                        ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                        ui.label(RichText::new(tweak.name).strong().size(16.0));
                                });

                                ui.label(RichText::new(tweak.description).small().weak());
                        });
                });
        }

        fn render_category_quick_actions(&mut self, ui: &mut egui::Ui, tweaks: &[&'static Tweak])
        {
                ui.horizontal(|ui| {
                        if ui.button("Enable All").clicked() {
                                for tweak in tweaks {
                                        self.tweak_states.states.insert(tweak.id.to_string(), true);
                                }
                                self.autosave();
                        }
                        if ui.button("Disable All").clicked() {
                                for tweak in tweaks {
                                        self.tweak_states.states.insert(tweak.id.to_string(), false);
                                }
                                self.autosave();
                        }
                });
        }

        fn render_category_header(&mut self, ui: &mut egui::Ui, category: Option<&super::tweaks::TweakCategory>)
        {
                if let Some(cat) = category {
                        ui.add_space(10.0);
                        ui.heading(RichText::new(cat.name).size(28.0));
                        ui.add_space(4.0);
                        ui.label(RichText::new(cat.description).weak());
                        ui.add_space(20.0);
                }
        }

        fn render_category_tweaks_list(&mut self, ui: &mut egui::Ui, tweaks: Vec<&'static Tweak>)
        {
                ui.vertical(|ui| {
                        ui.set_max_width(800.0);
                        // Tweaks list with card layout
                        let visible_tweaks = self.get_visible_tweaks(tweaks);

                        if visible_tweaks.is_empty() && !self.search_query.is_empty() {
                                ui.label(format!("No tweaks in this category match '{}'", self.search_query));
                                return;
                        }

                        for tweak in visible_tweaks {
                                self.render_tweak_tree_node(ui, tweak);
                        }
                });
        }

        fn render_category_view(&mut self, ui: &mut egui::Ui, category_id: &str)
        {
                let category = CATEGORIES.iter().find(|c| c.id == category_id);
                let tweaks = get_tweaks_for_category(category_id);

                self.render_category_header(ui, category);
                self.render_category_quick_actions(ui, &tweaks);

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(12.0);

                self.render_category_tweaks_list(ui, tweaks);
        }

        fn render_tweak_row(&mut self, ui: &mut egui::Ui, tweak: &'static Tweak)
        {
                let is_enabled = self.tweak_states.states.get(tweak.id).copied().unwrap_or(false);

                let mut frame = egui::Frame::group(ui.style())
                        .fill(ui.style().visuals.faint_bg_color)
                        .corner_radius(8.0)
                        .inner_margin(egui::Margin::symmetric(12, 6))
                        .stroke(egui::Stroke::new(
                                1.0,
                                ui.style().visuals.widgets.inactive.bg_stroke.color,
                        ));

                let (rect, response) =
                        ui.allocate_at_least(egui::vec2(ui.available_width(), 40.0), egui::Sense::click());

                if response.hovered() {
                        frame = frame.fill(ui.style().visuals.widgets.hovered.bg_fill);
                }

                ui.put(rect, |ui: &mut egui::Ui| {
                        frame.show(ui, |ui| self.render_tweak_row_inner(ui, tweak, is_enabled))
                                .response
                });

                if response.clicked() {
                        self.navigate_to(
                                ViewMode::Tweaks,
                                self.selection.selected_category.clone(),
                                Some(tweak.id.to_string()),
                        );
                }
        }

        fn render_tweak_technical_details(&self, ui: &mut egui::Ui, tweak: &'static Tweak)
        {
                ui.label(RichText::new("Technical Details").strong().size(16.0).weak());
                ui.add_space(4.0);

                let tech_bg = Color32::from_rgba_premultiplied(20, 20, 30, 200);

                egui::Frame::group(ui.style())
                        .fill(tech_bg)
                        .inner_margin(8.0)
                        .show(ui, |ui| {
                                ScrollArea::vertical()
                                        .max_height(f32::INFINITY)
                                        .auto_shrink([false, true])
                                        .show(ui, |ui| {
                                                ui.set_width(ui.available_width());
                                                
                                                let has_real_ops = tweak.enabled_ops.iter().any(|op| !op.subkey.is_empty());
                                                let has_command = tweak.command.is_some();
                                                let has_gpo = tweak.gpo_ops.is_some() && !tweak.gpo_ops.unwrap().is_empty();

                                                if tweak.custom_apply.is_some() && !has_real_ops && !has_command && !has_gpo {
                                                        ui.label(RichText::new("\u{2139} This tweak uses a custom operation (PowerShell/Shell API) to apply multiple changes.").weak().italics());
                                                }

                                                if let Some(cmd) = tweak.command {
                                                        widgets::render_command(ui, cmd);
                                                }

                                                if let Some(ops) = tweak.gpo_ops {
                                                        for op in ops {
                                                                widgets::render_gpo_op(ui, op);
                                                        }
                                                }

                                                for op in tweak.enabled_ops {
                                                        if op.subkey.is_empty() { continue; }
                                                        widgets::render_registry_op(ui, op);
                                                }
                                        });
                        });
        }

        fn render_tweak_metadata_panel(&mut self, ui: &mut egui::Ui, tweak: &'static Tweak)
        {
                widgets::render_tweak_description_column(ui, tweak);
        }

        fn render_tweak_subtweak_list(&mut self, ui: &mut egui::Ui, tweak: &'static Tweak)
        {
                if !tweak.sub_tweaks.is_empty() {
                        ui.label(RichText::new("Sub-items").strong().size(18.0));
                        ui.add_space(8.0);

                        egui::Frame::group(ui.style())
                                .fill(ui.style().visuals.faint_bg_color)
                                .inner_margin(8.0)
                                .show(ui, |ui| {
                                        ui.vertical(|ui| {
                                                for sub in tweak.sub_tweaks {
                                                        let mut sub_enabled = self
                                                                .tweak_states
                                                                .states
                                                                .get(sub.id)
                                                                .copied()
                                                                .unwrap_or(false);
                                                        if ui.checkbox(&mut sub_enabled, sub.name)
                                                                .on_hover_text(sub.description)
                                                                .changed()
                                                        {
                                                                self.tweak_states
                                                                        .states
                                                                        .insert(sub.id.to_string(), sub_enabled);
                                                                self.autosave();
                                                        }
                                                }
                                        });
                                });
                        ui.add_space(20.0);
                }
        }

        fn render_tweak_header(&mut self, ui: &mut egui::Ui, tweak: &'static Tweak)
        {
                if ui.button(RichText::new("\u{2190}  Back to category").strong())
                        .clicked()
                {
                        self.navigate_to(ViewMode::Tweaks, self.selection.selected_category.clone(), None);
                }
                ui.add_space(24.0);

                ui.heading(RichText::new(tweak.name).size(32.0).strong());
                ui.add_space(6.0);

                let is_enabled = self.tweak_states.states.get(tweak.id).copied().unwrap_or(false);
                let mut enabled = is_enabled;

                ui.horizontal(|ui| {
                        ui.label(RichText::new("Status:").size(18.0));
                        let label = if enabled { "Enabled" } else { "Disabled" };
                        if ui.checkbox(&mut enabled, label).changed() {
                                self.tweak_states.states.insert(tweak.id.to_string(), enabled);
                                self.autosave();
                        }
                });

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(12.0);
        }

        fn render_input_controls(&mut self, ui: &mut egui::Ui)
        {
                ui.horizontal(|ui| {
                        ui.label(RichText::new("Configuration:").strong().size(16.0));

                        // Toggle search with button
                        if ui.small_button("🔍 Find").clicked() {
                                self.input_search_visible = !self.input_search_visible;
                        }

                        // Handle Ctrl+F
                        if ui.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::F)) {
                                self.input_search_visible = true;
                        }
                });

                if self.input_search_visible {
                        ui.horizontal(|ui| {
                                ui.label("Find:");
                                ui.text_edit_singleline(&mut self.input_search_query);
                                if ui.button("X").clicked() {
                                        self.input_search_visible = false;
                                        self.input_search_query.clear();
                                }
                        });
                }
        }

        fn render_tweak_custom_input(&mut self, ui: &mut egui::Ui, tweak: &'static Tweak)
        {
                ui.add_space(12.0);
                ui.separator();
                ui.add_space(12.0);

                self.render_input_controls(ui);

                ui.add_space(4.0);

                let mut value = self
                        .tweak_states
                        .input_values
                        .get(tweak.id)
                        .cloned()
                        .unwrap_or_else(|| String::from(tweak.default_text.unwrap_or("")));

                let search_query = self.input_search_query.clone();
                let theme_color = ui.visuals().text_color();

                let mut layouter = move |ui: &egui::Ui, string: &dyn egui::TextBuffer, wrap_width: f32| {
                        let mut job = widgets::generate_highlight_job(string.as_str(), &search_query, theme_color);
                        job.wrap.max_width = wrap_width;
                        ui.painter().layout_job(job)
                };

                if ui.add(egui::TextEdit::multiline(&mut value)
                        .hint_text(tweak.default_text.unwrap_or("Enter value..."))
                        .desired_width(f32::INFINITY)
                        .layouter(&mut layouter))
                        .changed()
                {
                        self.tweak_states.input_values.insert(tweak.id.to_string(), value);
                        self.autosave();
                }
                ui.add_space(4.0);
                ui.label(RichText::new("Use semicolons to separate multiple entries")
                        .small()
                        .weak());
        }

        fn render_tweak_detail(&mut self, ui: &mut egui::Ui, tweak_id: &str)
        {
                let tweak = get_all_tweaks().into_iter().find(|t| t.id == tweak_id);

                let Some(tweak) = tweak else {
                        ui.label("Tweak not found");
                        return;
                };

                self.render_tweak_header(ui, tweak);

                self.render_tweak_metadata_panel(ui, tweak);

                ui.add_space(12.0);
                self.render_tweak_technical_details(ui, tweak);

                if tweak.has_custom_input {
                        self.render_tweak_custom_input(ui, tweak);
                }

                ui.add_space(12.0);
                ui.add_space(12.0);

                self.render_tweak_subtweak_list(ui, tweak);
        }

        fn render_confirm_buttons(
                &mut self,
                ui: &mut egui::Ui,
                confirm_text: &str,
                color: Option<Color32>,
                on_confirm: impl FnOnce(&mut Self, &egui::Context),
        )
        {
                ui.horizontal(|ui| {
                        ui.add_space(ui.available_width() / 2.0 - 160.0);

                        let mut button = egui::Button::new(RichText::new(confirm_text).strong().size(18.0))
                                .min_size(egui::vec2(180.0, 44.0));

                        if let Some(c) = color {
                                button = button.fill(c);
                        } else {
                                button = button.fill(ui.style().visuals.selection.stroke.color);
                        }

                        if ui.add(button).clicked() {
                                on_confirm(self, ui.ctx());
                        }

                        ui.add_space(16.0);

                        if ui.add(
                                egui::Button::new(RichText::new("Cancel").size(18.0)).min_size(egui::vec2(120.0, 44.0))
                        )
                        .clicked()
                        {
                                self.navigate_back();
                        }
                });
        }

        fn render_confirm_apply(&mut self, ui: &mut egui::Ui)
        {
                ui.vertical_centered(|ui| {
                        ui.add_space(60.0);
                        ui.heading(RichText::new("Ready to Optimize?").size(36.0).strong());
                        ui.add_space(16.0);

                        let enabled_count = self.count_enabled_tweaks();
                        ui.label(RichText::new(format!(
                                "You have selected {enabled_count} tweaks to apply to your system."
                        ))
                        .size(18.0)
                        .weak());

                        ui.add_space(8.0);
                        ui.label(RichText::new(
                                "\u{2139} An automatic System Restore point will be created before applying changes.",
                        )
                        .color(Color32::from_rgb(100, 200, 255)));

                        ui.add_space(40.0);

                        self.render_confirm_buttons(ui, "Apply Changes", None, |slf, ctx| {
                                // Check for existing restore point
                                // Command: Get-ComputerRestorePoint | Where-Object { $_.Description -eq 'W11Boost Auto-Restore' }
                                let (success, stdout, _) = crate::common::run_system_command_output(
                                        "powershell.exe",
                                        &[
                                                "-NoProfile",
                                                "-Command",
                                                "Get-ComputerRestorePoint | Where-Object { $_.Description -eq 'W11Boost Auto-Restore' }",
                                        ],
                                ).unwrap_or((false, String::new(), String::new()));

                                if success && !stdout.trim().is_empty() {
                                        // Found existing restore point
                                        slf.navigate_to(ViewMode::ConfirmRestorePoint, slf.selection.selected_category.clone(), slf.selection.selected_tweak.clone());
                                } else {
                                        // No existing restore point or error checking, proceed as usual
                                        slf.spawn_tweaks_worker(ctx.clone(), false);
                                        slf.navigate_back();
                                }
                        });
                });
        }

        fn render_confirm_restore_point(&mut self, ui: &mut egui::Ui)
        {
                ui.vertical_centered(|ui| {
                        ui.add_space(60.0);
                        ui.heading(RichText::new("Restore Point Exists").size(36.0).strong());
                        ui.add_space(16.0);

                        ui.label(
                                RichText::new("A 'W11Boost Auto-Restore' point was already created recently.")
                                        .size(18.0)
                                        .weak(),
                        );

                        ui.add_space(8.0);
                        ui.label("You can skip creating a new one to save time, or create another one just in case.");

                        ui.add_space(40.0);

                        ui.horizontal(|ui| {
                                ui.add_space(ui.available_width() / 2.0 - 230.0);

                                // Create New Button
                                if ui.add(egui::Button::new(RichText::new("Create New").strong().size(18.0))
                                        .min_size(egui::vec2(140.0, 44.0))
                                        .fill(ui.style().visuals.selection.stroke.color))
                                        .clicked()
                                {
                                        self.spawn_tweaks_worker(ui.ctx().clone(), false);
                                        // Navigate back twice (ConfirmRestorePoint -> ConfirmApply -> Tweaks)
                                        // Or just clear stack and go to Tweaks? navigate_back puts us in ConfirmApply which isn't ideal after starting.
                                        // Let's rely on the fact that we came from ConfirmApply, so popping once goes there.
                                        // But we want to go "back" to the main view usually.
                                        // Implementing "navigate_back" takes us to ConfirmApply.
                                        // Ideally we want to exit the confirmation flow.
                                        // Let's just pop twice manually or do what navigate_back does but twice?
                                        // For now, let's just go back to where we were before ConfirmApply
                                        // Which means popping twice from back_stack?
                                        // Actually spawn_tweaks_worker is async, we should probably just return to the main view.
                                        // Let's assume the previous flow: ConfirmApply -> navigate_back() (which was main view).
                                        // So we need to go back twice.
                                        self.navigate_back(); // To ConfirmApply
                                        self.navigate_back(); // To Main View
                                }

                                ui.add_space(16.0);

                                // Skip Button
                                if ui.add(
                                        egui::Button::new(RichText::new("Skip & Apply").strong().size(18.0))
                                                .min_size(egui::vec2(140.0, 44.0))
                                                .fill(Color32::from_rgb(100, 180, 100)), // Greenish
                                )
                                .clicked()
                                {
                                        self.spawn_tweaks_worker(ui.ctx().clone(), true);
                                        self.navigate_back();
                                        self.navigate_back();
                                }

                                ui.add_space(16.0);

                                // Cancel Button
                                if ui.add(egui::Button::new(RichText::new("Cancel").size(18.0))
                                        .min_size(egui::vec2(120.0, 44.0)))
                                        .clicked()
                                {
                                        self.navigate_back();
                                }
                        });
                });
        }

        fn render_confirm_unset_all(&mut self, ui: &mut egui::Ui)
        {
                ui.vertical_centered(|ui| {
                        ui.add_space(60.0);
                        ui.heading(RichText::new("Unset All Tweaks?").size(36.0).strong());
                        ui.add_space(16.0);

                        ui.label(
                                RichText::new("This will uncheck all currently selected tweaks in the UI.")
                                        .size(18.0)
                                        .weak(),
                        );
                        ui.add_space(8.0);
                        ui.label(RichText::new("Note: This does not apply changes to your system yet.").weak());

                        ui.add_space(40.0);

                        let unset_color = Color32::from_rgb(140, 60, 60);
                        self.render_confirm_buttons(ui, "Unset All", Some(unset_color), |slf, _| {
                                for state in slf.tweak_states.states.values_mut() {
                                        *state = false;
                                }
                                slf.autosave();
                                slf.navigate_back();
                        });
                });
        }

        fn render_confirm_load_defaults(&mut self, ui: &mut egui::Ui)
        {
                ui.vertical_centered(|ui| {
                        ui.add_space(60.0);
                        ui.heading(RichText::new("Load Default Tweaks?").size(36.0).strong());
                        ui.add_space(16.0);

                        ui.label(RichText::new("This will reset your current selection to the recommended defaults.").size(18.0).weak());
                        ui.add_space(8.0);
                        ui.label(RichText::new("All custom changes to your selection will be lost.").weak());

                        ui.add_space(40.0);

                        self.render_confirm_buttons(ui, "Load Defaults", None, |slf, _| {
                                slf.tweak_states = TweakStates::default();
                                slf.autosave();
                                slf.navigate_back();
                        });
                });
        }

        pub fn render_bottom_bar(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context)
        {
                let (is_running, progress, status) = {
                        let state = self.shared.lock().unwrap();
                        (state.is_running, state.progress, state.status_message.clone())
                };

                // Progress bar
                if is_running {
                        ui.add(egui::ProgressBar::new(progress).show_percentage().animate(true));
                        ui.add_space(8.0);
                }

                // Status label
                if !status.is_empty() {
                        ui.horizontal(|ui| {
                                ui.label(RichText::new("\u{2139}").strong().color(ui
                                        .style()
                                        .visuals
                                        .selection
                                        .stroke
                                        .color));
                                ui.label(RichText::new(&status).small());
                        });
                        ui.add_space(8.0);
                }

                self.render_bottom_buttons(ui, is_running);
        }

        fn render_bottom_buttons_nav(&mut self, ui: &mut egui::Ui)
        {
                if ui.button("Show Selected").clicked() {
                        self.navigate_to(
                                ViewMode::SelectedTweaks,
                                self.selection.selected_category.clone(),
                                self.selection.selected_tweak.clone(),
                        );
                }

                ui.separator();

                if ui.button("Save Config").clicked() {
                        self.save_config();
                }

                if ui.button("Load Config").clicked() {
                        self.load_config();
                }

                if ui.button("Load Defaults").clicked() {
                        self.navigate_to(
                                ViewMode::ConfirmLoadDefaults,
                                self.selection.selected_category.clone(),
                                self.selection.selected_tweak.clone(),
                        );
                }

                if ui.button("Unset all tweaks").clicked() {
                        self.navigate_to(
                                ViewMode::ConfirmUnsetAll,
                                self.selection.selected_category.clone(),
                                self.selection.selected_tweak.clone(),
                        );
                }

                ui.separator();

                let log_text = if self.show_log_panel { "Hide Log" } else { "Show Log" };
                if ui.button(log_text).clicked() {
                        self.show_log_panel = !self.show_log_panel;
                }
        }

        fn render_bottom_buttons(&mut self, ui: &mut egui::Ui, is_running: bool)
        {
                ui.horizontal_wrapped(|ui| {
                        if is_running {
                                ui.disable();
                        }

                        let enabled_count = self.count_enabled_tweaks();
                        let apply_text = format!("Apply {enabled_count} Tweaks");

                        if ui.add(egui::Button::new(RichText::new(apply_text).strong())
                                .min_size(egui::vec2(140.0, 32.0))
                                .fill(ui.style().visuals.selection.stroke.color.gamma_multiply(0.8)))
                                .clicked()
                                && enabled_count > 0
                        {
                                self.navigate_to(
                                        ViewMode::ConfirmApply,
                                        self.selection.selected_category.clone(),
                                        self.selection.selected_tweak.clone(),
                                );
                        }

                        self.render_bottom_buttons_nav(ui);
                });
        }

        fn render_log_header(&self, ui: &mut egui::Ui)
        {
                egui::Frame::NONE
                        .inner_margin(egui::Margin {
                                left: 0,
                                right: 16,
                                top: 0,
                                bottom: 0,
                        })
                        .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                        ui.heading("System Log");
                                });
                                ui.add_space(4.0);
                                ui.separator();
                        });
        }

        fn render_log_scroll_area(&self, ui: &mut egui::Ui, messages: &[String])
        {
                ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .stick_to_bottom(true)
                        .show(ui, |ui| {
                                ui.set_width(ui.available_width());
                                egui::Frame::NONE
                                        .inner_margin(egui::Margin {
                                                left: 0,
                                                right: 12,
                                                top: 0,
                                                bottom: 0,
                                        })
                                        .show(ui, |ui| {
                                                widgets::render_log_messages_list(ui, messages);
                                        });
                        });
        }

        pub fn render_log_panel(&self, ui: &mut egui::Ui)
        {
                self.render_log_header(ui);
                ui.add_space(8.0);

                let messages = {
                        let state = self.shared.lock().unwrap();
                        state.log_messages.clone()
                };

                egui::Frame::new()
                        .fill(ui.style().visuals.extreme_bg_color)
                        .corner_radius(8.0)
                        .inner_margin(egui::Margin {
                                left: 12,
                                right: 0,
                                top: 12,
                                bottom: 12,
                        })
                        .show(ui, |ui| {
                                self.render_log_scroll_area(ui, &messages);
                        });
        }

        fn render_log_panel_side(&self, ctx: &egui::Context, frame: egui::Frame)
        {
                if self.show_log_panel {
                        egui::SidePanel::right("log_panel")
                                .min_width(300.0)
                                .default_width(350.0)
                                .resizable(true)
                                .frame(frame)
                                .show(ctx, |ui| self.render_log_panel(ui));
                }
        }

        fn render_sidebar_side(&mut self, ctx: &egui::Context, frame: egui::Frame)
        {
                egui::SidePanel::left("sidebar")
                        .min_width(220.0)
                        .default_width(260.0)
                        .resizable(true)
                        .frame(frame)
                        .show(ctx, |ui| self.render_sidebar(ui));
        }

        fn render_bottom_panel(&mut self, ctx: &egui::Context, color: Color32)
        {
                egui::TopBottomPanel::bottom("bottom_bar")
                        .frame(egui::Frame::NONE.fill(color).inner_margin(egui::Margin {
                                left: 30,
                                right: 30,
                                top: 10,
                                bottom: 15,
                        }))
                        .show(ctx, |ui| self.render_bottom_bar(ui, ctx));
        }

        fn render_central_panel(&mut self, ctx: &egui::Context, color: Color32)
        {
                egui::CentralPanel::default()
                        .frame(egui::Frame::NONE.fill(color).inner_margin(egui::Margin {
                                left: 0,
                                right: 0,
                                top: 20,
                                bottom: 0,
                        }))
                        .show(ctx, |ui| {
                                if self.is_running() {
                                        ui.disable();
                                }
                                // Use a unique ID based on the current view state to persist scroll position
                                let scroll_id = if let Some(tweak) = &self.selection.selected_tweak {
                                        format!("scroll_tweak_{}", tweak)
                                } else if let Some(cat) = &self.selection.selected_category {
                                        format!("scroll_cat_{}", cat)
                                } else {
                                        match self.mode {
                                                ViewMode::Tweaks => "scroll_main".to_string(),
                                                ViewMode::SelectedTweaks => "scroll_selected".to_string(),
                                                _ => "scroll_other".to_string(),
                                        }
                                };

                                ScrollArea::vertical()
                                        .id_salt(scroll_id)
                                        .auto_shrink([false, false])
                                        .show(ui, |ui| {
                                                ui.set_min_width(ui.available_width());
                                                egui::Frame::NONE
                                                        .inner_margin(egui::Margin {
                                                                left: 30,
                                                                right: 30,
                                                                top: 0,
                                                                bottom: 0,
                                                        })
                                                        .show(ui, |ui| {
                                                                self.render_content(ui);
                                                        });
                                        });
                        });
        }
}

impl eframe::App for W11BoostApp
{
        fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame)
        {
                // Handle mouse navigation (Back/Forward)
                ctx.input(|i| {
                        if i.pointer.button_pressed(egui::PointerButton::Extra1) {
                                self.navigate_back();
                        } else if i.pointer.button_pressed(egui::PointerButton::Extra2) {
                                self.navigate_forward();
                        }
                });

                #[cfg(windows)]
                if !self.dark_mode_enforced {
                        enforce_dark_mode(frame);
                        self.dark_mode_enforced = true;
                }

                let sidebar_color = egui::Color32::from_rgba_premultiplied(14, 21, 32, 255);
                let main_bg_color = egui::Color32::from_rgba_premultiplied(18, 28, 41, 255);

                let side_panel_frame = egui::Frame::NONE
                        .fill(sidebar_color)
                        .inner_margin(egui::Margin::symmetric(16, 16));

                self.render_log_panel_side(ctx, side_panel_frame);
                self.render_sidebar_side(ctx, side_panel_frame);
                self.render_bottom_panel(ctx, main_bg_color);
                self.render_central_panel(ctx, main_bg_color);
        }
}
