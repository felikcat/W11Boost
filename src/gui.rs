// Feature modules
mod disable_copilot;
mod disable_recall;
mod disable_sleep;
mod minimize_forensics;
mod minimize_online_data_collection;
mod non_intrusive_tweaks;
mod remove_w11boost;
mod reset_windows_store;

use crate::common::{barrett_div, center, restore_from_backup, BARRETT_DIV_100, BARRETT_DIV_12};
use anyhow::{anyhow, Result};
use fltk::{
        app::{self},
        button::{Button, CheckButton},
        dialog,
        enums::{self, Align},
        frame::Frame,
        prelude::{GroupExt as _, WidgetBase as _, WidgetExt as _, WindowExt as _},
        window::Window,
};
use fltk_theme::{color_themes, ColorTheme};
use std::collections::HashMap;
use strum::IntoEnumIterator as _;
use strum_macros::EnumIter;

// =============================================================================
// Constants
// =============================================================================

const WINDOW_WIDTH: i32 = 640;
const WINDOW_HEIGHT: i32 = 480;
const TOP_PADDING: i32 = 4;
const FONT_PATH: &str = "C:\\Windows\\Fonts\\segoeui.ttf";
const DISPLAY_TIMEOUT_SUCCESS: f64 = 5.0;
const DISPLAY_TIMEOUT_ERROR: f64 = 10.0;

// =============================================================================
// Model: Application state and feature definitions
// =============================================================================

#[derive(Clone, Debug, PartialEq)]
enum ViewState
{
        MainMenu,
        Applying,
        Success,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, EnumIter, PartialOrd, Ord)]
enum FeatureType
{
        MinimizeForensics,
        MinimizeOnlineData,
        DisableRecall,
        DisableCopilot,
        DisableSleepAndHibernate,
        InstallMicrosoftStore,
}

struct FeatureDefinition
{
        label: &'static str,
        run_fn: fn() -> Result<()>,
        error_name: &'static str,
}

impl FeatureType
{
        fn definition(&self) -> FeatureDefinition
        {
                match self {
                        Self::MinimizeForensics => FeatureDefinition {
                                label: "Minimize forensics / local data",
                                run_fn: minimize_forensics::run,
                                error_name: "minimize_forensics",
                        },
                        Self::MinimizeOnlineData => FeatureDefinition {
                                label: "Minimize Microsoft online data",
                                run_fn: minimize_online_data_collection::run,
                                error_name: "minimize_online_data_collection",
                        },
                        Self::DisableRecall => FeatureDefinition {
                                label: "Disable Windows Recall",
                                run_fn: disable_recall::run,
                                error_name: "disable_recall",
                        },
                        Self::DisableCopilot => FeatureDefinition {
                                label: "Disable Windows Copilot",
                                run_fn: disable_copilot::run,
                                error_name: "disable_copilot",
                        },
                        Self::DisableSleepAndHibernate => FeatureDefinition {
                                label: "Disable sleep and hibernate",
                                run_fn: disable_sleep::run,
                                error_name: "disable_sleep",
                        },
                        Self::InstallMicrosoftStore => FeatureDefinition {
                                label: "Install Microsoft Store",
                                run_fn: reset_windows_store::run,
                                error_name: "reset_windows_store",
                        },
                }
        }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum ButtonType
{
        Apply,
        Remove,
}

// =============================================================================
// ViewModel: State management and business logic
// =============================================================================

struct ViewModel
{
        buttons: HashMap<ButtonType, Button>,
        checkboxes: HashMap<FeatureType, CheckButton>,
        status_display: Option<Frame>,
        current_view: ViewState,
}

impl ViewModel
{
        fn new() -> Self
        {
                Self {
                        checkboxes: HashMap::new(),
                        buttons: HashMap::new(),
                        status_display: None,
                        current_view: ViewState::MainMenu,
                }
        }

        fn set_ui_elements(
                &mut self,
                checkboxes: HashMap<FeatureType, CheckButton>,
                buttons: HashMap<ButtonType, Button>,
                status_display: Frame,
        )
        {
                self.checkboxes = checkboxes;
                self.buttons = buttons;
                self.status_display = Some(status_display);
        }

        fn set_view(&mut self, view: ViewState)
        {
                self.current_view = view;
                self.update_ui();
        }

        fn update_ui(&mut self)
        {
                match self.current_view {
                        ViewState::MainMenu => self.show_main_menu(),
                        ViewState::Applying => self.show_applying_screen(),
                        ViewState::Success => self.show_success_screen(),
                }
        }

        fn show_main_menu(&mut self)
        {
                for button in self.buttons.values_mut() {
                        button.show();
                }
                for checkbox in self.checkboxes.values_mut() {
                        checkbox.show();
                }
                if let Some(status) = self.status_display.as_mut() {
                        status.hide();
                }
                app::redraw();
        }

        fn hide_main_menu(&mut self)
        {
                for button in self.buttons.values_mut() {
                        button.hide();
                }
                for checkbox in self.checkboxes.values_mut() {
                        checkbox.hide();
                }
                if let Some(status) = self.status_display.as_mut() {
                        status.show();
                }
                app::redraw();
        }

        fn show_applying_screen(&mut self)
        {
                app::wait();
                self.hide_main_menu();

                if let Some(status) = self.status_display.as_mut() {
                        status.set_label("Applying W11Boost, please wait...");
                        status.redraw();
                }
                app::flush();
        }

        fn show_success_screen(&mut self)
        {
                app::wait();
                self.hide_main_menu();

                if let Some(status) = self.status_display.as_mut() {
                        status.set_label(
                                "W11Boost has successfully finished applying changes. Reboot for them to take full effect.",
                        );
                        status.redraw();
                }
                app::flush();

                app::add_timeout3(DISPLAY_TIMEOUT_SUCCESS, |_| {
                        fltk_observe::with_state_mut(|state: &mut Self| {
                                state.set_view(ViewState::MainMenu);
                        });
                });
        }

        fn show_error_screen(&mut self, message: &str)
        {
                app::wait();
                self.hide_main_menu();

                if let Some(status) = self.status_display.as_mut() {
                        status.set_label(&format!(
                                "W11Boost encountered an error, take a screenshot of this and post an issue.\n\n{message}"
                        ));
                        status.redraw();
                }
                app::flush();

                app::add_timeout3(DISPLAY_TIMEOUT_ERROR, |_| {
                        fltk_observe::with_state_mut(|state: &mut Self| {
                                state.set_view(ViewState::MainMenu);
                        });
                });
        }

        fn apply(&mut self, _btn: &Button)
        {
                let choice = dialog::choice2(
                        center().0,
                        center().1,
                        "Are you sure you want to apply W11Boost?",
                        "&Yes",
                        "&No",
                        "",
                );

                if choice != Some(0_i32) {
                        return;
                }

                self.set_view(ViewState::Applying);

                // Always run non-intrusive tweaks first
                if let Err(e) = non_intrusive_tweaks::run() {
                        self.show_error_screen(&format!("non_intrusive_tweaks failed: {e}"));
                        return;
                }

                let mut feature_types: Vec<FeatureType> = FeatureType::iter().collect();
                feature_types.sort();

                for feature_type in feature_types {
                        let Some(checkbox) = self.checkboxes.get(&feature_type) else {
                                continue;
                        };

                        if !checkbox.is_checked() {
                                continue;
                        }

                        let definition = feature_type.definition();
                        if let Err(e) = (definition.run_fn)() {
                                self.show_error_screen(&format!("{} failed: {e}", definition.error_name));
                                return;
                        }
                }

                self.set_view(ViewState::Success);
        }

        fn remove(&mut self, _btn: &Button)
        {
                let choice = dialog::choice2(
                        center().0,
                        center().1,
                        "Are you sure you want to uninstall W11Boost?",
                        "&Yes",
                        "&No",
                        "",
                );

                if choice != Some(0_i32) {
                        return;
                }

                self.set_view(ViewState::Applying);

                if let Err(e) = remove_w11boost::run() {
                        self.show_error_screen(&format!("remove_w11boost::run failed: {e}"));
                        return;
                }

                match restore_from_backup() {
                        Ok(()) => self.set_view(ViewState::Success),
                        Err(e) => self.show_error_screen(&format!("restore_from_backup failed: {e}")),
                }
        }
}

// =============================================================================
// View: UI element creation and layout
// =============================================================================

struct View
{
        buttons: HashMap<ButtonType, Button>,
        checkboxes: HashMap<FeatureType, CheckButton>,
        status_display: Frame,
}

impl View
{
        fn new() -> Self
        {
                let mut window = Window::default()
                        .with_label("W11Boost")
                        .with_size(WINDOW_WIDTH, WINDOW_HEIGHT)
                        .center_screen();
                window.set_border(true);

                let checkboxes = Self::create_checkboxes();
                let buttons = Self::create_buttons();
                let status_display = Self::create_status_display();

                Self::setup_window_drag(&mut window);
                window.end();
                window.show();
                window.center_screen();

                Self {
                        buttons,
                        checkboxes,
                        status_display,
                }
        }

        fn create_checkboxes() -> HashMap<FeatureType, CheckButton>
        {
                let checkbox_height = barrett_div(WINDOW_HEIGHT, &BARRETT_DIV_12);
                let checkbox_width = WINDOW_WIDTH >> 1;
                let mut checkboxes = HashMap::new();

                for (index, feature_type) in FeatureType::iter().enumerate() {
                        let definition = feature_type.definition();
                        let y = TOP_PADDING + checkbox_height * (index as i32) + (index as i32 * 2);

                        let mut checkbox = CheckButton::new(0, y, checkbox_width, checkbox_height, definition.label);
                        checkbox.set_label_font(enums::Font::by_name(FONT_PATH));
                        checkbox.set_label_size(16);

                        checkboxes.insert(feature_type, checkbox);
                }

                checkboxes
        }

        fn create_buttons() -> HashMap<ButtonType, Button>
        {
                let button_height = barrett_div(WINDOW_HEIGHT * 14, &BARRETT_DIV_100);
                let button_width = (WINDOW_WIDTH - 4) >> 1;

                let mut apply = Button::new(0, 0, button_width, button_height, "Apply W11Boost");
                apply.set_pos(2, WINDOW_HEIGHT - button_height - 2);
                apply.set_label_font(enums::Font::by_name(FONT_PATH));
                apply.set_label_size(16);

                let mut remove = Button::new(0, 0, button_width, button_height, "Remove W11Boost");
                remove.set_pos(WINDOW_WIDTH - button_width - 2, WINDOW_HEIGHT - button_height - 2);
                remove.set_label_font(enums::Font::by_name(FONT_PATH));
                remove.set_label_size(16);

                let mut buttons = HashMap::new();
                buttons.insert(ButtonType::Apply, apply);
                buttons.insert(ButtonType::Remove, remove);
                buttons
        }

        fn create_status_display() -> Frame
        {
                let mut status_display = Frame::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT, None);
                status_display.set_label_size(16);
                status_display.set_align(Align::Center | Align::Inside | Align::Wrap);
                status_display.hide();
                status_display
        }

        fn setup_window_drag(window: &mut Window)
        {
                window.handle({
                        let (mut x, mut y) = (0_i32, 0_i32);
                        move |w, ev| match ev {
                                enums::Event::Push => {
                                        let coords = app::event_coords();
                                        x = coords.0;
                                        y = coords.1;
                                        true
                                }
                                enums::Event::Drag => {
                                        w.set_pos(app::event_x_root() - x, app::event_y_root() - y);
                                        true
                                }
                                _ => false,
                        }
                });
        }
}

// =============================================================================
// Application: Orchestrates View and ViewModel
// =============================================================================

struct Application
{
        app: app::App,
}

impl Application
{
        fn new() -> Result<Self>
        {
                use fltk_observe::{Runner as _, WidgetObserver as _};

                let app = app::App::default()
                        .with_scheme(app::Scheme::Gtk)
                        .use_state(ViewModel::new)
                        .ok_or_else(|| anyhow!("Failed to initialize app with state."))?;

                app.load_font(FONT_PATH)?;

                let widget_theme = ColorTheme::new(color_themes::BLACK_THEME);
                widget_theme.apply();

                let mut view = View::new();

                if let Some(apply_btn) = view.buttons.get_mut(&ButtonType::Apply) {
                        apply_btn.set_action(ViewModel::apply);
                }

                if let Some(remove_btn) = view.buttons.get_mut(&ButtonType::Remove) {
                        remove_btn.set_action(ViewModel::remove);
                }

                fltk_observe::with_state_mut(|state: &mut ViewModel| {
                        state.set_ui_elements(view.checkboxes, view.buttons, view.status_display);
                });

                Ok(Self { app })
        }

        fn run(&self) -> Result<()>
        {
                self.app
                        .run()
                        .map_err(|e| anyhow!("Failed to run Application.\nError: {}", e))?;
                Ok(())
        }
}

// =============================================================================
// Public API
// =============================================================================

pub fn draw_gui() -> Result<()>
{
        let app = Application::new().map_err(|e| anyhow!("Failed to initialize Application.\nError: {}", e))?;
        app.run()
}
