mod defaults;
mod disable_recall;
mod disable_sleep;
mod disable_vbs;
mod reduce_forensics;
mod reduce_online_data_collection;
mod remove_w11boost;

use crate::common::center;
use fltk::{
        app::{self, Screen},
        button::{Button, CheckButton},
        dialog,
        draw::{self},
        enums::{self, Align, Color},
        frame::Frame,
        prelude::{DisplayExt, GroupExt, WidgetBase, WidgetExt, WindowExt},
        text::{TextBuffer, TextDisplay},
        widget::Widget,
        window::Window,
};
use fltk_theme::{ColorTheme, color_themes};
use std::{error::Error, mem, process::exit, thread::sleep, time::Duration};
use windows::Win32::{
        Foundation::HWND,
        UI::WindowsAndMessaging::{HWND_TOPMOST, SWP_FRAMECHANGED, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW, SetWindowPos},
};

const WINDOW_WIDTH: i32 = 640;
const WINDOW_HEIGHT: i32 = 480;
const TITLEBAR_HEIGHT: i32 = 32;
const FONT_PATH: &str = "C:\\Windows\\Fonts\\segoeui.ttf";

#[derive(Debug, Clone, PartialEq)]
enum ViewState {
        MainMenu,
        Applying,
}

struct GuiViewModel {
        minimize_forensics: Option<CheckButton>,
        minimize_online_data: Option<CheckButton>,
        apply_btn: Option<Button>,
        remove_btn: Option<Button>,
        status_display: Option<Frame>,
        current_view: ViewState,
}
impl GuiViewModel {
        fn new() -> Self {
                Self {
                        minimize_forensics: None,
                        minimize_online_data: None,
                        apply_btn: None,
                        remove_btn: None,
                        status_display: None,
                        current_view: ViewState::MainMenu,
                }
        }

        fn update_ui(&mut self) {
                match self.current_view {
                        ViewState::MainMenu => self.toggle_main_screen(true),
                        ViewState::Applying => self.show_applying_screen(),
                }
        }

        fn set_view(&mut self, view: ViewState) {
                self.current_view = view;
                self.update_ui();
        }

        fn toggle_main_screen(&mut self, visible: bool) {
                if let (Some(apply), Some(remove), Some(minimize_forensics), Some(minimize_online_data), Some(status)) = (
                        self.apply_btn.as_mut(),
                        self.remove_btn.as_mut(),
                        self.minimize_forensics.as_mut(),
                        self.minimize_online_data.as_mut(),
                        self.status_display.as_mut(),
                ) {
                        // This is implicit, since there are no other use cases
                        if visible {
                                apply.show();
                                remove.show();
                                minimize_forensics.show();
                                minimize_online_data.show();
                                status.hide();
                        } else {
                                apply.hide();
                                remove.hide();
                                minimize_forensics.hide();
                                minimize_online_data.hide();
                                status.show();
                        }
                        app::redraw(); // Screen changed
                }
        }

        fn show_error_screen(&mut self, message: String) {
                app::wait();
                self.toggle_main_screen(false);

                if let Some(status) = self.status_display.as_mut() {
                        status.show();
                        status.set_label(&format!(
                                "W11Boost encountered an error, read its log file for more information.\n\n{}",
                                message
                        ));
                        status.set_label_size(16);
                        status.redraw();
                }

                // Force GUI to process all pending events and render
                app::flush();
                // Could draw a visual timer at some point
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
        }

        fn set_ui_elements(
                &mut self,
                minimize_forensics: CheckButton,
                minimize_online_data: CheckButton,
                apply_btn: Button,
                remove_btn: Button,
                status_display: Frame,
        ) {
                self.minimize_forensics = Some(minimize_forensics);
                self.minimize_online_data = Some(minimize_online_data);
                self.apply_btn = Some(apply_btn);
                self.remove_btn = Some(remove_btn);
                self.status_display = Some(status_display);
        }

        fn apply(&mut self, _btn: &Button) {
                let choice = dialog::choice2(center().0, center().1, "Are you sure you want to apply W11Boost?", "&Yes", "&No", "");

                if choice == Some(0) {
                        let (forensics_checked, online_data_checked) = {
                                if let (Some(minimize_forensics), Some(minimize_online_data)) =
                                        (self.minimize_forensics.as_mut(), self.minimize_online_data.as_mut())
                                {
                                        (minimize_forensics.is_checked(), minimize_online_data.is_checked())
                                } else {
                                        return;
                                }
                        };

                        self.set_view(ViewState::Applying);

                        if forensics_checked {
                                if let Err(e) = reduce_forensics::run() {
                                        self.show_error_screen(format!("reduce_forensics failed: {}", e));
                                } else {
                                        self.set_view(ViewState::MainMenu);
                                }
                        }
                        if online_data_checked {
                                if let Err(e) = reduce_online_data_collection::run() {
                                        self.show_error_screen(format!("reduce_online_data_collection failed: {}", e));
                                }
                                else {
                                        self.set_view(ViewState::MainMenu);
                                }
                        }

                        
                }
        }
}
struct GuiView {
        apply: Button,
        remove: Button,
        minimize_forensics: CheckButton,
        minimize_online_data: CheckButton,
        status_display: Frame,
}
impl GuiView {
        fn new() -> Self {
                let mut wind = Window::default().with_label("W11Boost").with_size(WINDOW_WIDTH, WINDOW_HEIGHT).center_screen();

                wind.set_border(false);

                let mut titlebar = Frame::new(0, 0, WINDOW_WIDTH, 32, None);

                titlebar.set_frame(enums::FrameType::FlatBox);
                titlebar.set_color(Color::from_rgb(22, 22, 22));

                let mut titlebar_close = Button::new(WINDOW_WIDTH - 32, 0, 32, 32, "X");
                titlebar_close.set_frame(enums::FrameType::NoBox);
                titlebar_close.set_callback(move |_| exit(0));

                let mut apply = Button::new(0, 0, (WINDOW_WIDTH - 6) / 2, (WINDOW_HEIGHT * 14) / 100, "Apply W11Boost");

                let mut remove = Button::new(WINDOW_WIDTH / 2, 0, (WINDOW_WIDTH - 6) / 2, (WINDOW_HEIGHT * 14) / 100, "Remove W11Boost");

                let apply_height = apply.height();
                apply.set_pos(2, WINDOW_HEIGHT - apply_height - 2);
                apply.set_label_font(enums::Font::by_name(FONT_PATH));
                apply.set_label_size(16);

                let remove_width = remove.width();
                let remove_height = remove.height();
                remove.set_pos(WINDOW_WIDTH - remove_width - 2, WINDOW_HEIGHT - remove_height - 2);
                remove.set_label_font(enums::Font::by_name(FONT_PATH));
                remove.set_label_size(16);

                let checkbox_height = WINDOW_HEIGHT / 12;

                let mut minimize_forensics = CheckButton::new(0, TITLEBAR_HEIGHT, WINDOW_WIDTH / 2, checkbox_height, "Minimize local data / forensics");
                let mut minimize_online_data = CheckButton::new(
                        0,
                        TITLEBAR_HEIGHT + checkbox_height + 2,
                        WINDOW_WIDTH / 2,
                        checkbox_height,
                        "Minimize Microsoft online data",
                );

                let mut my_checkboxes = [&mut minimize_forensics, &mut minimize_online_data];

                for checkbox in &mut my_checkboxes {
                        checkbox.set_label_font(enums::Font::by_name(FONT_PATH));
                        checkbox.set_label_size(16);
                }

                let mut status_display = Frame::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT, None);
                status_display.set_label_size(24);
                status_display.set_align(Align::Center | Align::Inside | Align::Wrap);
                status_display.hide();

                wind.end();
                wind.show();

                let hwnd = wind.raw_handle();
                let hwnd: HWND = unsafe { mem::transmute(hwnd) };

                unsafe {
                        // Always on top
                        SetWindowPos(
                                hwnd,
                                Some(HWND_TOPMOST),
                                0,
                                0,
                                0,
                                0,
                                SWP_SHOWWINDOW | SWP_FRAMECHANGED | SWP_NOMOVE | SWP_NOSIZE,
                        )
                        .unwrap();
                }

                // Only accounts for the primary monitor
                let screen = Screen::new(0).expect("Could not find screen");
                wind.resize((screen.w() - wind.width()) / 2, (screen.h() - wind.height()) / 2, wind.width(), wind.height());

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

                Self {
                        apply,
                        remove,
                        minimize_forensics,
                        minimize_online_data,
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
                let app = app::App::default().with_scheme(app::Scheme::Gtk).use_state(|| GuiViewModel::new()).unwrap();

                app.load_font("C:\\Windows\\Fonts\\segoeui.ttf").unwrap();

                let widget_theme = ColorTheme::new(color_themes::BLACK_THEME);
                widget_theme.apply();

                let mut gv = GuiView::new();

                gv.apply.set_action(GuiViewModel::apply);

                fltk_observe::with_state_mut(|state: &mut GuiViewModel| {
                        state.set_ui_elements(gv.minimize_forensics, gv.minimize_online_data, gv.apply, gv.remove, gv.status_display);
                });
                Self { app }
        }

        fn run(&self) {
                self.app.run().unwrap();
        }
}

pub fn draw_gui() -> Result<(), Box<dyn Error>> {
        let ga = GuiApp::new();
        ga.run();

        Ok(())
}
