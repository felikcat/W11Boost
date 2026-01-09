use eframe::egui::{self, Color32, RichText, ScrollArea};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use std::sync::{Arc, Mutex};

use super::config::{TweakConfig, get_config_dir};
use super::shared_state::{SharedState, WorkerContext};
use super::state::{SelectionState, TweakStates, ViewMode};
use super::theme;
use super::tweaks::{CATEGORIES, Tweak, apply_tweak, get_all_tweaks, get_tweaks_for_category};

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

        fn spawn_tweaks_worker(&self, ctx: egui::Context)
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

        fn spawn_remove_worker(&self, ctx: egui::Context)
        {
                let enabled_tweaks = self.get_enabled_tweaks();
                let total_ops = enabled_tweaks.len() as u32;
                let shared = Arc::clone(&self.shared);

                // Collect tweaks to revert
                let tweaks_to_revert: Vec<&'static Tweak> = enabled_tweaks;
                let input_values = self.tweak_states.input_values.clone();

                std::thread::spawn(move || {
                        let worker_ctx = WorkerContext::new(shared, ctx, total_ops, input_values);
                        let result = crate::gui::remove_w11boost::run_revert(&worker_ctx, tweaks_to_revert);

                        match result {
                                Ok(()) => worker_ctx.post_complete(),
                                Err(e) => worker_ctx.post_error(format!("{e}")),
                        }
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

        /// Render the left sidebar with category tree
        pub fn render_sidebar(&mut self, ui: &mut egui::Ui)
        {
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
                                        self.selection.selected_category = None;
                                        self.selection.selected_tweak = None;
                                        self.mode = ViewMode::Tweaks;
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
                        self.selection.selected_category = Some(category.id.to_string());
                        self.selection.selected_tweak = None;
                        self.mode = ViewMode::Tweaks;

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
                                        self.selection.selected_category = Some(category_id.to_string());
                                        self.selection.selected_tweak = Some(tweak.id.to_string());
                                        self.mode = ViewMode::Tweaks;
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
                        ViewMode::ConfirmRemove => {
                                self.render_confirm_remove(ui);
                        }
                        ViewMode::ConfirmUnsetAll => {
                                self.render_confirm_unset_all(ui);
                        }
                        ViewMode::SelectedTweaks => {
                                self.render_selected_tweaks(ui);
                        }
                }
        }

        fn render_selected_tweaks(&mut self, ui: &mut egui::Ui)
        {
                if ui.button(RichText::new("\u{2190}  Back to Tweaks").strong()).clicked() {
                        self.mode = ViewMode::Tweaks;
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
                        self.selection.selected_category = Some(category.id.to_string());
                        self.selection.selected_tweak = None;
                        self.mode = ViewMode::Tweaks;
                }
        }

        fn render_welcome(&mut self, ui: &mut egui::Ui)
        {
                ui.add_space(40.0);
                ui.vertical_centered(|ui| {
                        ui.heading(RichText::new("Welcome to W11Boost").size(32.0).strong());
                        ui.add_space(12.0);
                        ui.label(RichText::new("A sizeable customization suite.").size(16.0).weak());
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
                                        Self::render_effect_badge(ui, tweak.effect);
                                });

                                ui.label(RichText::new(tweak.description).small().weak());
                        });
                });
        }

        fn render_category_quick_actions(&mut self, ui: &mut egui::Ui, tweaks: &[&'static Tweak]) {
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

        fn render_category_header(&mut self, ui: &mut egui::Ui, category: Option<&super::tweaks::TweakCategory>) {
                 if let Some(cat) = category {
                        ui.add_space(10.0);
                        ui.heading(RichText::new(cat.name).size(28.0));
                        ui.add_space(4.0);
                        ui.label(RichText::new(cat.description).weak());
                        ui.add_space(20.0);
                }
        }

        fn render_category_tweaks_list(&mut self, ui: &mut egui::Ui, tweaks: Vec<&'static Tweak>) {
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

        fn render_effect_badge(ui: &mut egui::Ui, effect: super::tweaks::TweakEffect)
        {
                let (effect_text, effect_color) = match effect {
                        super::tweaks::TweakEffect::Immediate => ("Immediate", egui::Color32::from_rgb(100, 220, 100)),
                        super::tweaks::TweakEffect::ExplorerRestart => {
                                ("Explorer Restart", egui::Color32::from_rgb(255, 180, 50))
                        }
                        super::tweaks::TweakEffect::Logoff => ("Logoff", egui::Color32::from_rgb(255, 120, 50)),
                        super::tweaks::TweakEffect::Restart => ("Restart", egui::Color32::from_rgb(255, 80, 80)),
                };

                ui.add_space(4.0);
                ui.label(RichText::new(effect_text).small().color(effect_color).strong());
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
                        self.selection.selected_tweak = Some(tweak.id.to_string());
                }
        }

        fn render_tweak_technical_details(&self, ui: &mut egui::Ui, tweak: &'static Tweak)
        {
                ui.label(RichText::new("Technical Details (Registry Changes)")
                        .strong()
                        .size(16.0)
                        .weak());
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
                                                if tweak.custom_apply.is_some() && !has_real_ops {
                                                        ui.label(RichText::new("\u{2139} This tweak uses a custom operation (PowerShell/Shell API) to apply multiple changes.").weak().italics());
                                                }

                                                for op in tweak.enabled_ops {
                                                        if op.subkey.is_empty() { continue; }
                                                        self.render_registry_op(ui, op);
                                                }
                                        });
                        });
        }

        fn render_registry_op(&self, ui: &mut egui::Ui, op: &super::tweaks::RegistryOp)
        {
                let value_str = match &op.value {
                        super::tweaks::RegistryValue::Dword(v) => format!("{v} (DWORD)"),
                        super::tweaks::RegistryValue::String(s) => format!("\"{s}\" (String)"),
                        super::tweaks::RegistryValue::ExpandSz(s) => format!("\"{s}\" (ExpandString)"),
                        super::tweaks::RegistryValue::Binary(v) => format!("{v:?} (Binary)"),
                        super::tweaks::RegistryValue::Delete => "DELETE".to_string(),
                        super::tweaks::RegistryValue::DeleteKey => "DELETE KEY".to_string(),
                };

                let stock_str = match &op.stock_value {
                        super::tweaks::RegistryValue::Dword(v) => format!("{v} (DWORD)"),
                        super::tweaks::RegistryValue::String(s) => format!("\"{s}\" (String)"),
                        super::tweaks::RegistryValue::ExpandSz(s) => format!("\"{s}\" (ExpandString)"),
                        super::tweaks::RegistryValue::Binary(v) => format!("{v:?} (Binary)"),
                        super::tweaks::RegistryValue::Delete => "NOT PRESENT (Value)".to_string(),
                        super::tweaks::RegistryValue::DeleteKey => "NOT PRESENT (Key)".to_string(),
                };

                ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new(format!("{}:", op.hkey))
                                .strong()
                                .monospace()
                                .size(14.0)
                                .color(ui.style().visuals.selection.stroke.color));
                        ui.label(RichText::new(format!("{}\\{}", op.subkey, op.value_name))
                                .monospace()
                                .size(14.0)
                                .weak());
                });

                ui.horizontal_wrapped(|ui| {
                        ui.add_space(8.0);
                        ui.label(RichText::new("Target:")
                                .monospace()
                                .size(14.0)
                                .color(Color32::from_rgb(200, 200, 200)));
                        ui.label(RichText::new(format!(" = {}", value_str))
                                .monospace()
                                .size(14.0)
                                .color(Color32::from_rgb(115, 179, 242)));
                });

                ui.horizontal_wrapped(|ui| {
                        ui.add_space(8.0);
                        ui.label(RichText::new("Stock: ")
                                .monospace()
                                .size(14.0)
                                .color(Color32::from_rgb(200, 200, 200)));
                        ui.label(RichText::new(format!(" = {}", stock_str))
                                .monospace()
                                .size(14.0)
                                .color(Color32::from_rgb(255, 165, 0)));
                });

                ui.add_space(4.0);
        }

        fn render_tweak_description_column(&self, ui: &mut egui::Ui, tweak: &'static Tweak) {
                ui.vertical(|ui| {
                        ui.label(RichText::new("Description").strong().size(16.0));
                        ui.add_space(4.0);
                        egui::Frame::group(ui.style())
                                .fill(ui.style().visuals.faint_bg_color)
                                .inner_margin(8.0)
                                .show(ui, |ui| {
                                        ui.label(tweak.description);
                                });
                });
        }

        fn render_tweak_impact_column(&self, ui: &mut egui::Ui, tweak: &'static Tweak) {
               ui.vertical(|ui| {
                        ui.label(RichText::new("Impact & Effect").strong().size(16.0));
                        ui.add_space(4.0);
                        egui::Frame::group(ui.style())
                                .fill(ui.style().visuals.faint_bg_color)
                                .inner_margin(8.0)
                                .show(ui, |ui| {
                                        let desc = tweak.effect.description();
                                        let is_logoff = desc == "Requires sign out/sign in";

                                        if desc != "Requires restart" && !is_logoff {
                                                ui.label(desc);
                                        }
                                        if tweak.requires_restart {
                                                if desc != "Requires restart" {
                                                        ui.add_space(2.0);
                                                }
                                                ui.label(RichText::new("\u{26A0} Requires restart")
                                                        .color(Color32::from_rgb(255, 100, 100)));
                                        }
                                        if is_logoff {
                                                ui.label(RichText::new("\u{26A0} Requires sign out/sign in")
                                                        .color(Color32::from_rgb(255, 190, 40)));
                                        }
                                });
                });
        }

        fn render_tweak_metadata_panel(&mut self, ui: &mut egui::Ui, tweak: &'static Tweak)
        {
                ui.columns(2, |columns| {
                        self.render_tweak_description_column(&mut columns[0], tweak);
                        self.render_tweak_impact_column(&mut columns[1], tweak);
                });
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
                        self.selection.selected_tweak = None;
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

        fn render_input_controls(&mut self, ui: &mut egui::Ui) {
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

        fn generate_highlight_job(text: &str, search_query: &str, theme_color: Color32) -> egui::text::LayoutJob {
                let mut layout_job = egui::text::LayoutJob::default();
                // We use auto wrapping with infinite width because we want multiline editing,
                // but usually layouters are called with a specific wrap width.
                // However, our logic appending text is independent of wrap width except for
                // the fact that we're feeding it to the painter eventually.
                // The `layouter` closure in egui is responsible for setting the max_width.
                // We'll leave that for the caller to set on the result or pass in.
                // Actually, `layout_job` holds the sections. Wrapping is applied by the widget
                // using the job's break settings.
                
                if search_query.is_empty() {
                        layout_job.append(
                                text,
                                0.0,
                                egui::TextFormat {
                                        font_id: egui::FontId::monospace(14.0),
                                        color: theme_color,
                                        ..Default::default()
                                },
                        );
                } else {
                        let haystack = text.to_lowercase();
                        let needle = search_query.to_lowercase();
                        let mut last_end = 0;

                        for (start, part) in haystack.match_indices(&needle) {
                                if start > last_end {
                                        layout_job.append(
                                                &text[last_end..start],
                                                0.0,
                                                egui::TextFormat {
                                                        font_id: egui::FontId::monospace(14.0),
                                                        color: theme_color,
                                                        ..Default::default()
                                                },
                                        );
                                }

                                let match_end = start + part.len();
                                layout_job.append(
                                        &text[start..match_end],
                                        0.0,
                                        egui::TextFormat {
                                                font_id: egui::FontId::monospace(14.0),
                                                background: Color32::from_rgb(255, 255, 0),
                                                color: Color32::BLACK,
                                                ..Default::default()
                                        },
                                );

                                last_end = match_end;
                        }

                        if last_end < text.len() {
                                layout_job.append(
                                        &text[last_end..],
                                        0.0,
                                        egui::TextFormat {
                                                font_id: egui::FontId::monospace(14.0),
                                                color: theme_color,
                                                ..Default::default()
                                        },
                                );
                        }
                }
                layout_job
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
                        let mut job = Self::generate_highlight_job(string.as_str(), &search_query, theme_color);
                        job.wrap.max_width = wrap_width;
                        ui.painter().layout_job(job)
                };

                if ui.add(
                        egui::TextEdit::multiline(&mut value)
                                .hint_text(tweak.default_text.unwrap_or("Enter value..."))
                                .desired_width(f32::INFINITY)
                                .layouter(&mut layouter),
                )
                .changed()
                {
                        self.tweak_states.input_values.insert(tweak.id.to_string(), value);
                        self.autosave();
                }
                ui.add_space(4.0);
                ui.label(RichText::new("Use semicolons to separate multiple entries").small().weak());
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
                                self.mode = ViewMode::Tweaks;
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
                                "\u{26A0} It is highly recommended to create a System Restore point first.",
                        )
                        .color(Color32::from_rgb(255, 180, 50)));

                        ui.add_space(40.0);

                        self.render_confirm_buttons(ui, "Apply Changes", None, |slf, ctx| {
                                slf.spawn_tweaks_worker(ctx.clone());
                                slf.mode = ViewMode::Tweaks;
                        });
                });
        }

        fn render_confirm_remove(&mut self, ui: &mut egui::Ui)
        {
                ui.vertical_centered(|ui| {
                        ui.add_space(60.0);
                        ui.heading(RichText::new("Remove W11Boost?").size(36.0).strong());
                        ui.add_space(16.0);

                        ui.label(RichText::new(
                                "This will attempt to revert all registry changes made by this application.",
                        )
                        .size(18.0)
                        .weak());
                        ui.add_space(8.0);
                        ui.label(RichText::new("Are you sure you want to continue?")
                                .strong()
                                .color(Color32::from_rgb(255, 100, 100)));

                        ui.add_space(40.0);

                        let remove_color = Color32::from_rgb(140, 60, 60);
                        self.render_confirm_buttons(ui, "Confirm Removal", Some(remove_color), |slf, ctx| {
                                slf.spawn_remove_worker(ctx.clone());
                                slf.mode = ViewMode::Tweaks;
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
                                slf.mode = ViewMode::Tweaks;
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

        fn render_bottom_buttons_nav(&mut self, ui: &mut egui::Ui) {
             if ui.button("Remove W11Boost").clicked() {
                    self.mode = ViewMode::ConfirmRemove;
            }

            if ui.button("Show Selected").clicked() {
                    self.mode = ViewMode::SelectedTweaks;
            }

            ui.separator();

            if ui.button("Save Config").clicked() {
                    self.save_config();
            }

            if ui.button("Load Config").clicked() {
                    self.load_config();
            }

            if ui.button("Unset all tweaks").clicked() {
                    self.mode = ViewMode::ConfirmUnsetAll;
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
                                self.mode = ViewMode::ConfirmApply;
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

        fn render_log_messages_list(&self, ui: &mut egui::Ui, messages: &[String])
        {
                if messages.is_empty() {
                        ui.weak("No log messages yet.");
                } else {
                        for msg in messages {
                                ui.label(RichText::new(msg).monospace().size(12.0));
                        }
                }
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
                                                self.render_log_messages_list(ui, messages);
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
                                ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
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
