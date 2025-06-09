mod disable_recall;
mod disable_sleep;
mod install_appx_support;
mod minimize_forensics;
mod minimize_online_data_collection;
mod non_intrusive_tweaks;
mod reset_windows_store;

use crate::common::center;
use fltk::{
        app::{self},
        button::{Button, CheckButton},
        dialog,
        enums::{self, Align},
        frame::Frame,
        prelude::{GroupExt, WidgetBase, WidgetExt, WindowExt},
        window::Window,
};
use fltk_theme::{ColorTheme, color_themes};
use std::sync::OnceLock;
use std::{collections::HashMap, error::Error};

const WINDOW_WIDTH: i32 = 640;
const WINDOW_HEIGHT: i32 = 480;
const TOP_PADDING: i32 = 4;
const FONT_PATH: &str = "C:\\Windows\\Fonts\\segoeui.ttf";

#[derive(Debug, Clone, PartialEq)]
enum ViewState {
        MainMenu,
        Applying,
        Success,
}

struct CheckboxConfig {
        label: &'static str,
        run_fn: fn() -> Result<(), Box<dyn std::error::Error>>,
        error_name: &'static str,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum CheckboxType {
        MinimizeForensics,
        MinimizeOnlineData,
        DisableRecall,
        DisableSleepAndHibernate,
        InstallMicrosoftStore,
        NonIntrusiveTweaks,
        InstallAppxSupport,
}

static CHECKBOX_CONFIGS: OnceLock<HashMap<CheckboxType, CheckboxConfig>> = OnceLock::new();

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ButtonType {
        Apply,
}

struct GuiViewModel {
        checkboxes: HashMap<CheckboxType, CheckButton>,
        buttons: HashMap<ButtonType, Button>,
        status_display: Option<Frame>,
        current_view: ViewState,
}
impl GuiViewModel {
        fn new() -> Self {
                Self {
                        checkboxes: HashMap::new(),
                        buttons: HashMap::new(),
                        status_display: None,
                        current_view: ViewState::MainMenu,
                }
        }

        fn update_ui(&mut self) {
                match self.current_view {
                        ViewState::MainMenu => self.toggle_main_screen(true),
                        ViewState::Applying => self.show_applying_screen(),
                        ViewState::Success => self.show_success_screen(),
                }
        }

        fn set_view(&mut self, view: ViewState) {
                self.current_view = view;
                self.update_ui();
        }

        fn toggle_main_screen(&mut self, visible: bool) {
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

        fn show_success_screen(&mut self) {
                app::wait();
                self.toggle_main_screen(false);

                if let Some(status) = self.status_display.as_mut() {
                        status.show();
                        status.set_label(&format!(
                                "W11Boost has successfully finished applying changes. Reboot for them to take full effect."
                        ));
                        status.redraw();
                }

                app::flush();
                app::add_timeout3(5.0, |_| {
                        fltk_observe::with_state_mut(|state: &mut GuiViewModel| {
                                state.set_view(ViewState::MainMenu);
                        });
                });
        }

        fn show_error_screen(&mut self, message: String) {
                app::wait();
                self.toggle_main_screen(false);

                if let Some(status) = self.status_display.as_mut() {
                        status.show();
                        status.set_label(&format!(
                                "W11Boost encountered an error, take a screenshot of this and post an issue.\n\n{}",
                                message
                        ));
                        status.redraw();
                }

                // Force GUI to process all pending events and render
                app::flush();

                app::add_timeout3(10.0, |_| {
                        fltk_observe::with_state_mut(|state: &mut GuiViewModel| {
                                state.set_view(ViewState::MainMenu);
                        });
                });
        }
        fn show_applying_screen(&mut self) {
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
        ) {
                self.checkboxes = checkboxes;
                self.buttons = buttons;
                self.status_display = Some(status_display);
        }

        fn apply(&mut self, _btn: &Button) {
                let choice = dialog::choice2(
                        center().0,
                        center().1,
                        "Are you sure you want to apply W11Boost?",
                        "&Yes",
                        "&No",
                        "",
                );

                if choice == Some(0) {
                        let checkbox_configs = GuiApp::get_checkbox_configs();

                        self.set_view(ViewState::Applying);

                        for (checkbox_type, checkbox_config) in checkbox_configs {
                                let Some(checkbox) = self.checkboxes.get(checkbox_type) else {
                                        continue;
                                };

                                if !checkbox.is_checked() {
                                        continue;
                                }

                                if let Err(e) = (checkbox_config.run_fn)() {
                                        self.show_error_screen(format!("{} failed: {}", checkbox_config.error_name, e));
                                        return;
                                }
                        }
                        
                        self.set_view(ViewState::Success);
                }
        }
}
struct GuiView {
        buttons: HashMap<ButtonType, Button>,
        checkboxes: HashMap<CheckboxType, CheckButton>,
        status_display: Frame,
}
impl GuiView {
        fn new() -> Self {
                let mut wind = Window::default()
                        .with_label("W11Boost")
                        .with_size(WINDOW_WIDTH, WINDOW_HEIGHT)
                        .center_screen();

                wind.set_border(true);

                let mut apply = Button::new(
                        0,
                        0,
                        WINDOW_WIDTH - 4,
                        (WINDOW_HEIGHT * 14) / 100,
                        "Apply W11Boost",
                );

                let apply_height = apply.height();
                apply.set_pos(2, WINDOW_HEIGHT - apply_height - 2);
                apply.set_label_font(enums::Font::by_name(FONT_PATH));
                apply.set_label_size(16);

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
                status_display.set_label_size(24);
                status_display.set_align(Align::Center | Align::Inside | Align::Wrap);
                status_display.hide();

                let mut buttons = HashMap::new();
                buttons.insert(ButtonType::Apply, apply);

                wind.end();
                wind.show();

                wind.handle({
                        let (mut x, mut y) = (0, 0);
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
                        checkboxes,
                        buttons,
                        status_display,
                }
        }
}

struct GuiApp {
        app: app::App,
}
impl GuiApp {
        fn new() -> Self {
                use fltk_observe::{Runner, WidgetObserver};
                let app = app::App::default()
                        .with_scheme(app::Scheme::Gtk)
                        .use_state(|| GuiViewModel::new())
                        .unwrap();

                app.load_font("C:\\Windows\\Fonts\\segoeui.ttf").unwrap();

                let widget_theme = ColorTheme::new(color_themes::BLACK_THEME);
                widget_theme.apply();

                let mut gv = GuiView::new();

                if let Some(apply_btn) = gv.buttons.get_mut(&ButtonType::Apply) {
                        apply_btn.set_action(GuiViewModel::apply);
                }

                fltk_observe::with_state_mut(|state: &mut GuiViewModel| {
                        state.set_ui_elements(gv.checkboxes, gv.buttons, gv.status_display);
                });
                Self { app }
        }

        fn run(&self) {
                self.app.run().unwrap();
        }

        fn get_checkbox_configs() -> &'static HashMap<CheckboxType, CheckboxConfig> {
                let checkbox_height = WINDOW_HEIGHT / 12;

                CHECKBOX_CONFIGS.get_or_init(|| {
                        let mut map = HashMap::new();

                        map.insert(
                                CheckboxType::MinimizeForensics,
                                CheckboxConfig {
                                        label: "Minimize forensics / local data",
                                        run_fn: minimize_forensics::run,
                                        error_name: "minimize_forensics",
                                        x: 0,
                                        y: TOP_PADDING,
                                        width: WINDOW_WIDTH / 2,
                                        height: checkbox_height,
                                },
                        );

                        map.insert(
                                CheckboxType::MinimizeOnlineData,
                                CheckboxConfig {
                                        label: "Minimize Microsoft online data",
                                        run_fn: minimize_online_data_collection::run,
                                        error_name: "minimize_online_data_collection",
                                        x: 0,
                                        y: TOP_PADDING + checkbox_height + 2,
                                        width: WINDOW_WIDTH / 2,
                                        height: checkbox_height,
                                },
                        );

                        map.insert(
                                CheckboxType::DisableRecall,
                                CheckboxConfig {
                                        label: "Disable Windows Recall",
                                        run_fn: disable_recall::run,
                                        error_name: "disable_recall",
                                        x: 0,
                                        y: TOP_PADDING + checkbox_height * 2 + 4,
                                        width: WINDOW_WIDTH / 2,
                                        height: checkbox_height,
                                },
                        );

                        map.insert(
                                CheckboxType::DisableSleepAndHibernate,
                                CheckboxConfig {
                                        label: "Disable sleep and hibernate",
                                        run_fn: disable_sleep::run,
                                        error_name: "disable_sleep",
                                        x: 0,
                                        y: TOP_PADDING + checkbox_height * 3 + 6,
                                        width: WINDOW_WIDTH / 2,
                                        height: checkbox_height,
                                },
                        );

                        map.insert(
                                CheckboxType::InstallMicrosoftStore,
                                CheckboxConfig {
                                        label: "Install Microsoft Store",
                                        run_fn: reset_windows_store::run,
                                        error_name: "reset_windows_store",
                                        x: 0,
                                        y: TOP_PADDING + checkbox_height * 4 + 8,
                                        width: WINDOW_WIDTH / 2,
                                        height: checkbox_height,
                                },
                        );

                        map.insert(
                                CheckboxType::NonIntrusiveTweaks,
                                CheckboxConfig {
                                        label: "Use non-intrusive tweaks",
                                        run_fn: non_intrusive_tweaks::run,
                                        error_name: "non_intrusive_tweaks",
                                        x: 0,
                                        y: TOP_PADDING + checkbox_height * 5 + 10,
                                        width: WINDOW_WIDTH / 2,
                                        height: checkbox_height,
                                },
                        );

                        map.insert(
                                CheckboxType::InstallAppxSupport,
                                CheckboxConfig {
                                        label: "Install support for UWP and WinGet",
                                        run_fn: install_appx_support::run,
                                        error_name: "install_appx_support",
                                        x: 0,
                                        y: TOP_PADDING + checkbox_height * 6 + 12,
                                        width: WINDOW_WIDTH / 2,
                                        height: checkbox_height,
                                },
                        );

                        map
                })
        }
}

pub fn draw_gui() -> Result<(), Box<dyn Error>> {
        let ga = GuiApp::new();
        ga.run();

        Ok(())
}
