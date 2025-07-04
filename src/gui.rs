mod disable_copilot;
mod disable_recall;
mod disable_sleep;
mod install_appx_support;
mod minimize_forensics;
mod minimize_online_data_collection;
mod non_intrusive_tweaks;
mod remove_w11boost;
mod reset_windows_store;

use crate::common::{barrett_div, BARRETT_DIV_100, BARRETT_DIV_12};
use crate::common::{center, restore_from_backup};
use anyhow::Result;
use anyhow::anyhow;
use fltk::{
        app::{self},
        button::{Button, CheckButton},
        dialog,
        enums::{self, Align},
        frame::Frame,
        prelude::{GroupExt as _, WidgetBase as _, WidgetExt as _, WindowExt as _},
        window::Window,
};
use fltk_theme::{ColorTheme, color_themes};
use std::collections::HashMap;
use std::sync::OnceLock;
use strum::IntoEnumIterator as _;
use strum_macros::EnumIter;

const WINDOW_WIDTH: i32 = 640;
const WINDOW_HEIGHT: i32 = 480;
const TOP_PADDING: i32 = 4;
const FONT_PATH: &str = "C:\\Windows\\Fonts\\segoeui.ttf";

#[derive(Clone, PartialEq)]
enum ViewState
{
        Applying,
        MainMenu,
        Success,
}

struct CheckboxConfig
{
        label: &'static str,
        run_fn: fn() -> Result<()>,
        error_name: &'static str,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
}

#[derive(Clone, PartialEq, Eq, Hash, EnumIter, PartialOrd, Ord)]
enum CheckboxType
{
        DisableCopilot,
        DisableRecall,
        DisableSleepAndHibernate,
        InstallAppxSupport,
        InstallMicrosoftStore,
        MinimizeForensics,
        MinimizeOnlineData,
        NonIntrusiveTweaks,
}

impl CheckboxType
{
        fn config(&self) -> CheckboxConfig
        {
                let checkbox_height = barrett_div(WINDOW_HEIGHT, &BARRETT_DIV_12);
                let checkbox_width = WINDOW_WIDTH >> 1_i32; // 50% of window.

                match self {
                        Self::MinimizeForensics => CheckboxConfig {
                                label: "Minimize forensics / local data",
                                run_fn: minimize_forensics::run,
                                error_name: "minimize_forensics",
                                x: 0,
                                y: TOP_PADDING,
                                width: checkbox_width,
                                height: checkbox_height,
                        },
                        Self::MinimizeOnlineData => CheckboxConfig {
                                label: "Minimize Microsoft online data",
                                run_fn: minimize_online_data_collection::run,
                                error_name: "minimize_online_data_collection",
                                x: 0,
                                y: TOP_PADDING + checkbox_height + 2,
                                width: checkbox_width,
                                height: checkbox_height,
                        },
                        Self::DisableRecall => CheckboxConfig {
                                label: "Disable Windows Recall",
                                run_fn: disable_recall::run,
                                error_name: "disable_recall",
                                x: 0,
                                y: TOP_PADDING + checkbox_height * 2 + 4,
                                width: checkbox_width,
                                height: checkbox_height,
                        },
                        Self::DisableCopilot => CheckboxConfig {
                                label: "Disable Windows Copilot",
                                run_fn: disable_copilot::run,
                                error_name: "disable_recall",
                                x: 0,
                                y: TOP_PADDING + checkbox_height * 3 + 6,
                                width: checkbox_width,
                                height: checkbox_height,
                        },
                        Self::DisableSleepAndHibernate => CheckboxConfig {
                                label: "Disable sleep and hibernate",
                                run_fn: disable_sleep::run,
                                error_name: "disable_sleep",
                                x: 0,
                                y: TOP_PADDING + checkbox_height * 4 + 8,
                                width: checkbox_width,
                                height: checkbox_height,
                        },
                        Self::InstallMicrosoftStore => CheckboxConfig {
                                label: "Install Microsoft Store",
                                run_fn: reset_windows_store::run,
                                error_name: "reset_windows_store",
                                x: 0,
                                y: TOP_PADDING + checkbox_height * 5 + 10,
                                width: checkbox_width,
                                height: checkbox_height,
                        },
                        Self::NonIntrusiveTweaks => CheckboxConfig {
                                label: "Use non-intrusive tweaks",
                                run_fn: non_intrusive_tweaks::run,
                                error_name: "non_intrusive_tweaks",
                                x: 0,
                                y: TOP_PADDING + checkbox_height * 6 + 12,
                                width: checkbox_width,
                                height: checkbox_height,
                        },
                        Self::InstallAppxSupport => CheckboxConfig {
                                label: "Install support for UWP and WinGet",
                                run_fn: install_appx_support::run,
                                error_name: "install_appx_support",
                                x: 0,
                                y: TOP_PADDING + checkbox_height * 7 + 14,
                                width: checkbox_width,
                                height: checkbox_height,
                        },
                }
        }
}

static CHECKBOX_CONFIGS: OnceLock<HashMap<CheckboxType, CheckboxConfig>> = OnceLock::new();

#[derive(Clone, PartialEq, Eq, Hash)]
enum ButtonType
{
        Apply,
        Remove,
}

const DISPLAY_TIMEOUT_SUCCESS: f64 = 5.0;
const DISPLAY_TIMEOUT_ERROR: f64 = 10.0;

struct GuiViewModel
{
        buttons: HashMap<ButtonType, Button>,
        checkboxes: HashMap<CheckboxType, CheckButton>,
        current_view: ViewState,
        status_display: Option<Frame>,
}
impl GuiViewModel
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

        fn update_ui(&mut self)
        {
                match self.current_view {
                        ViewState::MainMenu => self.toggle_main_screen(true),
                        ViewState::Applying => self.show_applying_screen(),
                        ViewState::Success => self.show_success_screen(),
                }
        }

        fn set_view(&mut self, view: ViewState)
        {
                self.current_view = view;
                self.update_ui();
        }

        fn toggle_main_screen(&mut self, visible: bool)
        {
                if let (Some(status),) = (self.status_display.as_mut(),) {
                        // This is implicit, since there are no other use cases
                        for button in self.buttons.values_mut() {
                                if visible {
                                        button.show();
                                } else {
                                        button.hide();
                                }
                        }

                        for checkbox in self.checkboxes.values_mut() {
                                if visible {
                                        checkbox.show();
                                } else {
                                        checkbox.hide();
                                }
                        }

                        if visible {
                                status.hide();
                        } else {
                                status.show();
                        }
                        app::redraw(); // Screen changed
                }
        }

        fn show_success_screen(&mut self)
        {
                app::wait();
                self.toggle_main_screen(false);

                if let Some(status) = self.status_display.as_mut() {
                        status.show();
                        status.set_label("W11Boost has successfully finished applying changes. Reboot for them to take full effect.");
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
                self.toggle_main_screen(false);

                if let Some(status) = self.status_display.as_mut() {
                        status.show();
                        status.set_label(&format!(
                                "W11Boost encountered an error, take a screenshot of this and post an issue.\n\n{message}"
                        ));
                        status.redraw();
                }

                // Force GUI to process all pending events and render
                app::flush();

                app::add_timeout3(DISPLAY_TIMEOUT_ERROR, |_| {
                        fltk_observe::with_state_mut(|state: &mut Self| {
                                state.set_view(ViewState::MainMenu);
                        });
                });
        }
        fn show_applying_screen(&mut self)
        {
                app::wait(); // Applying view won't render correctly otherwise
                self.toggle_main_screen(false);

                if let Some(status) = self.status_display.as_mut() {
                        status.show();
                        status.set_label("Applying W11Boost, please wait...");
                        status.redraw();
                }

                app::flush();
        }

        fn set_ui_elements(
                &mut self,
                checkboxes: HashMap<CheckboxType, CheckButton>,
                buttons: HashMap<ButtonType, Button>,
                status_display: Frame,
        )
        {
                self.checkboxes = checkboxes;
                self.buttons = buttons;
                self.status_display = Some(status_display);
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

                if choice == Some(0_i32) {
                        let checkbox_configs = GuiApp::get_checkbox_configs();
                        let mut checkbox_types = checkbox_configs.keys().cloned().collect::<Vec<_>>();
                        checkbox_types.sort();

                        self.set_view(ViewState::Applying);

                        for checkbox_type in checkbox_types {
                                let checkbox_config = &checkbox_configs[&checkbox_type];
                                let Some(checkbox) = self.checkboxes.get(&checkbox_type) else {
                                        continue;
                                };

                                if !checkbox.is_checked() {
                                        continue;
                                }

                                if let Err(e) = (checkbox_config.run_fn)() {
                                        self.show_error_screen(&format!("{} failed: {e}", checkbox_config.error_name));
                                        return;
                                }
                        }

                        self.set_view(ViewState::Success);
                }
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

                if choice == Some(0_i32) {
                        self.set_view(ViewState::Applying);

                        match remove_w11boost::run() {
                                Ok(()) => {}
                                Err(e) => {
                                        self.show_error_screen(&format!("remove_w11boost::run failed: {e}"));
                                        return;
                                }
                        }

                        match restore_from_backup() {
                                Ok(()) => {
                                        self.set_view(ViewState::Success);
                                }
                                Err(e) => {
                                        self.show_error_screen(&format!("restore_from_backup failed: {e}"));
                                }
                        }
                }
        }
}
struct GuiView
{
        buttons: HashMap<ButtonType, Button>,
        checkboxes: HashMap<CheckboxType, CheckButton>,
        status_display: Frame,
}
impl GuiView
{
        fn new() -> Self
        {
                let mut wind = Window::default()
                        .with_label("W11Boost")
                        .with_size(WINDOW_WIDTH, WINDOW_HEIGHT)
                        .center_screen();

                wind.set_border(true);

                let mut apply = Button::new(
                        0,
                        0,
                        (WINDOW_WIDTH - 4) >> 1_i32,
                        barrett_div(WINDOW_HEIGHT * 14, &BARRETT_DIV_100),
                        "Apply W11Boost",
                );

                let apply_height = apply.height();
                apply.set_pos(2, WINDOW_HEIGHT - apply_height - 2);
                apply.set_label_font(enums::Font::by_name(FONT_PATH));
                apply.set_label_size(16);

                let mut remove = Button::new(
                        WINDOW_WIDTH >> 1_i32,
                        0,
                        (WINDOW_WIDTH - 4) >> 1_i32,
                        barrett_div(WINDOW_HEIGHT * 14, &BARRETT_DIV_100),
                        "Remove W11Boost",
                );
                let remove_width = remove.width();
                let remove_height = remove.height();
                remove.set_pos(WINDOW_WIDTH - remove_width - 2, WINDOW_HEIGHT - remove_height - 2);
                remove.set_label_font(enums::Font::by_name(FONT_PATH));
                remove.set_label_size(16);

                let checkbox_configs = GuiApp::get_checkbox_configs();
                let mut checkboxes = HashMap::new();

                for (checkbox_type, config) in checkbox_configs {
                        let mut checkbox =
                                CheckButton::new(config.x, config.y, config.width, config.height, config.label);

                        checkbox.set_label_font(enums::Font::by_name(FONT_PATH));
                        checkbox.set_label_size(16);
                        checkboxes.insert(checkbox_type.clone(), checkbox);
                }

                if let Some(checkbox) = checkboxes.get_mut(&CheckboxType::NonIntrusiveTweaks) {
                        checkbox.set_checked(true);
                }

                let mut status_display = Frame::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT, None);
                status_display.set_label_size(16);
                status_display.set_align(Align::Center | Align::Inside | Align::Wrap);
                status_display.hide();

                let mut buttons = HashMap::new();
                buttons.insert(ButtonType::Apply, apply);
                buttons.insert(ButtonType::Remove, remove);

                wind.end();
                wind.show();

                wind.handle({
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

                wind.center_screen();

                Self {
                        buttons,
                        checkboxes,
                        status_display,
                }
        }
}

struct GuiApp
{
        app: app::App,
}
impl GuiApp
{
        fn new() -> Result<Self>
        {
                use fltk_observe::{Runner as _, WidgetObserver as _};
                let app = app::App::default()
                        .with_scheme(app::Scheme::Gtk)
                        .use_state(GuiViewModel::new)
                        .ok_or_else(|| anyhow!("Failed to initialize app with state."))?;

                app.load_font("C:\\Windows\\Fonts\\segoeui.ttf")?;

                let widget_theme = ColorTheme::new(color_themes::BLACK_THEME);
                widget_theme.apply();

                let mut gv = GuiView::new();

                if let Some(apply_btn) = gv.buttons.get_mut(&ButtonType::Apply) {
                        apply_btn.set_action(GuiViewModel::apply);
                }

                if let Some(remove_btn) = gv.buttons.get_mut(&ButtonType::Remove) {
                        remove_btn.set_action(GuiViewModel::remove);
                }

                fltk_observe::with_state_mut(|state: &mut GuiViewModel| {
                        state.set_ui_elements(gv.checkboxes, gv.buttons, gv.status_display);
                });

                Ok(Self { app })
        }

        fn run(&self) -> Result<()>
        {
                self.app.run()
                        .map_err(|e| anyhow!("Failed to run GuiApp.\nError: {}", e))?;
                Ok(())
        }

        fn get_checkbox_configs() -> &'static HashMap<CheckboxType, CheckboxConfig>
        {
                CHECKBOX_CONFIGS.get_or_init(|| CheckboxType::iter().map(|ct| (ct.clone(), ct.config())).collect())
        }
}

pub fn draw_gui() -> Result<()>
{
        let ga = GuiApp::new().map_err(|e| anyhow!("Failed to initialize GuiApp in draw_gui.\nError: {}", e))?;
        ga.run()?;

        Ok(())
}
