mod appx_support;
mod create_system_restore_point;
mod defaults;
mod disable_defender_and_smartscreen;
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
    enums::{self, Color},
    frame::{self},
    prelude::*,
    widget::Widget,
    window::Window,
};
use fltk_theme::{ColorTheme, color_themes};
use std::{
    cell::RefCell,
    error::Error,
    mem,
    process::{Command, exit},
    rc::Rc,
};
use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{
        HWND_TOPMOST, SWP_FRAMECHANGED, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW, SetWindowPos,
    },
};

type MyCheckboxes = Vec<RefCell<CheckButton>>;

pub fn draw_gui() -> Result<(), Box<dyn Error>> {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let font = app.load_font("C:\\Windows\\Fonts\\segoeui.ttf").unwrap();

    let mut wind = Window::default()
        .with_label("W11Boost")
        .with_size(640, 480)
        .center_screen();

    wind.set_border(false);

    let widget_theme = ColorTheme::new(color_themes::BLACK_THEME);
    widget_theme.apply();

    let mut titlebar = frame::Frame::new(0, 0, wind.width(), 32, None);

    titlebar.set_frame(enums::FrameType::FlatBox);
    titlebar.set_color(Color::from_rgb(22, 22, 22));

    let mut titlebar_close = Button::new(wind.width() - 32, 0, 32, 32, "X");
    titlebar_close.set_frame(enums::FrameType::NoBox);
    titlebar_close.set_callback(move |_| exit(0));

    let apply = Rc::new(RefCell::new(
        Button::new(
            0,
            0,
            (wind.width() - 6) / 2,
            (wind.height() * 14) / 100,
            "Apply W11Boost",
        )
        .center_of(&wind),
    ));

    let remove = Rc::new(RefCell::new(Button::new(
        wind.width() / 2,
        0,
        (wind.width() - 6) / 2,
        (wind.height() * 14) / 100,
        "Remove W11Boost",
    )));

    {
        let mut apply_mut = apply.borrow_mut();
        let apply_height = apply_mut.height();
        apply_mut.set_pos(2, wind.height() - apply_height - 2);
        apply_mut.set_label_font(enums::Font::by_name(&font));
        apply_mut.set_label_size(16);
    }

    {
        let mut remove_mut = remove.borrow_mut();
        remove_mut.clone().center_of(&wind);
        let remove_width = remove_mut.width();
        let remove_height = remove_mut.height();
        remove_mut.set_pos(
            wind.width() - remove_width - 2,
            wind.height() - remove_height - 2,
        );
        remove_mut.set_label_font(enums::Font::by_name(&font));
        remove_mut.set_label_size(16);
    }

    let checkbox_height = wind.height() / 12;

    let my_checkboxes: MyCheckboxes = [
        RefCell::new(CheckButton::new(
            0,
            titlebar.height(),
            wind.width() / 2,
            checkbox_height,
            "Minimize local data / forensics",
        )),
        RefCell::new(CheckButton::new(
            0,
            titlebar.height() + checkbox_height + 2,
            wind.width() / 2,
            checkbox_height,
            "Minimize Microsoft online data",
        )),
        RefCell::new(CheckButton::new(
            0,
            titlebar.height() + checkbox_height * 2 + 4,
            wind.width() / 2,
            checkbox_height,
            "Create a system restore point",
        )),
        RefCell::new(CheckButton::new(
            0,
            titlebar.height() + checkbox_height * 3 + 6,
            wind.width() / 2,
            checkbox_height,
            "Install the Microsoft Store",
        )),
        RefCell::new(CheckButton::new(
            0,
            titlebar.height() + checkbox_height * 4 + 8,
            wind.width() / 2,
            checkbox_height,
            "Install support for UWP and WinGet",
        )),
        RefCell::new(CheckButton::new(
            0,
            titlebar.height() + checkbox_height * 5 + 10,
            wind.width() / 2,
            checkbox_height,
            "Disable Defender and SmartScreen",
        )),
        RefCell::new(CheckButton::new(
            0,
            titlebar.height() + checkbox_height * 6 + 12,
            wind.width() / 2,
            checkbox_height,
            "Disable sleep and hibernate",
        )),
        RefCell::new(CheckButton::new(
            0,
            titlebar.height() + checkbox_height * 7 + 14,
            wind.width() / 2,
            checkbox_height,
            "Disable Virtualization Based Security",
        )),
        RefCell::new(CheckButton::new(
            0,
            titlebar.height() + checkbox_height * 8 + 16,
            wind.width() / 2,
            checkbox_height,
            "Add in non-intrusive tweaks",
        )),
        RefCell::new(CheckButton::new(
            wind.width() / 2,
            titlebar.height(),
            wind.width() / 2,
            checkbox_height,
            "Disable Windows Recall",
        )),
    ]
    .to_vec();

    for checkbox in &my_checkboxes {
        let mut checkbox = checkbox.borrow_mut();
        checkbox.set_label_font(enums::Font::by_name(&font));
        checkbox.set_label_size(16);
    }

    my_checkboxes[2].borrow_mut().set_value(true); // "Create a system restore point"
    my_checkboxes[8].borrow_mut().set_value(true); // "Add in non-intrusive tweaks"

    let frame0 = Rc::new(RefCell::new(
        Widget::default()
            .with_size(wind.width(), wind.height() - titlebar.height())
            .with_pos(0, titlebar.height()),
    ));

    {
        let frame0 = Rc::clone(&frame0);
        frame0.borrow_mut().set_frame(enums::FrameType::BorderBox);
        let font = font.clone();
        frame0.borrow_mut().draw(move |f| {
            let label = f.label();
            let txt = label.split("  ").nth(0).unwrap();
            let x = f.x();
            let y = f.y();
            let w = f.w();
            let h = f.h();

            draw::push_clip(x, y, w, h);
            draw::draw_box(f.frame(), x, y, w, h, f.color());
            draw::set_draw_color(f.label_color());
            draw::set_font(enums::Font::by_name(&font), 16);
            draw::draw_text2(txt, x, y - 16, w, h, f.align());
            draw::pop_clip();
        });
        frame0
            .borrow_mut()
            .set_label("Applying W11Boost, please wait...");
        frame0.borrow_mut().hide();
    }

    wind.end();
    wind.show();

    let hwnd = wind.raw_handle();
    let hwnd: HWND = unsafe { mem::transmute(hwnd) };

    unsafe {
        // Always on top
        SetWindowPos(
            hwnd,
            HWND_TOPMOST,
            0,
            0,
            0,
            0,
            SWP_SHOWWINDOW | SWP_FRAMECHANGED | SWP_NOMOVE | SWP_NOSIZE,
        )?;
    }

    // Only accounts for the primary monitor
    let screen = Screen::new(0).expect("Could not find screen");
    wind.resize(
        (screen.w() - wind.width()) / 2,
        (screen.h() - wind.height()) / 2,
        wind.width(),
        wind.height(),
    );

    wind.handle({
        let mut x = 0;
        let mut y = 0;
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

    fn hide_elements(
        frame0: &Rc<RefCell<Widget>>,
        apply: &Rc<RefCell<Button>>,
        remove: &Rc<RefCell<Button>>,
        my_checkboxes: &[RefCell<CheckButton>],
    ) {
        frame0.borrow_mut().show();
        frame0.borrow_mut().top_window();

        // So these aren't accidentally clicked.
        apply.borrow_mut().hide();
        remove.borrow_mut().hide();
        for checkbox in my_checkboxes {
            checkbox.borrow_mut().hide();
        }

        // Force window to redraw to display frame0.
        app::flush();
        app::wait();
    }

    fn show_elements(
        frame0: &Rc<RefCell<Widget>>,
        apply: &Rc<RefCell<Button>>,
        remove: &Rc<RefCell<Button>>,
        my_checkboxes: &[RefCell<CheckButton>],
    ) {
        // Does not require a manual redraw.
        frame0.borrow_mut().hide();
        apply.borrow_mut().show();
        remove.borrow_mut().show();
        for checkbox in my_checkboxes {
            checkbox.borrow_mut().show();
        }
    }

    {
        let frame0 = Rc::clone(&frame0);
        let apply_cloned = Rc::clone(&apply);
        let remove_cloned = Rc::clone(&remove);
        let my_checkboxes_cloned = my_checkboxes.clone();
        apply.borrow_mut().set_callback(move |_| {
            let choice = dialog::choice2(
                center().0,
                center().1,
                "Are you sure you want to apply W11Boost?",
                "&Yes",
                "&No",
                "",
            );
            if choice == Some(0) {
                hide_elements(
                    &frame0,
                    &apply_cloned,
                    &remove_cloned,
                    &my_checkboxes_cloned,
                );
                // Has to be first!
                if let Ok(checkbox) = my_checkboxes_cloned[2].try_borrow() {
                    if checkbox.is_checked() {
                        create_system_restore_point::run()
                            .expect("create_system_restore_point::run failed");
                    }
                }

                if let Ok(checkbox) = my_checkboxes_cloned[0].try_borrow() {
                    if checkbox.is_checked() {
                        reduce_forensics::run().expect("reduce_forensics::run failed");
                    }
                }
                if let Ok(checkbox) = my_checkboxes_cloned[1].try_borrow() {
                    if checkbox.is_checked() {
                        reduce_online_data_collection::run()
                            .expect("reduce_online_data_collection::run failed");
                    }
                }
                if let Ok(checkbox) = my_checkboxes_cloned[3].try_borrow() {
                    if checkbox.is_checked() {
                        let mut child = Command::new("wsreset.exe")
                            .args(&["-i"])
                            .spawn()
                            .expect("wsreset.exe failed to execute");

                        child.wait().expect("wsreset.exe -> failed to wait for the child process");
                    }
                }
                if let Ok(checkbox) = my_checkboxes_cloned[4].try_borrow() {
                    if checkbox.is_checked() {
                        appx_support::install().expect("appx_support::install failed");
                    }
                }

                if let Ok(checkbox) = my_checkboxes_cloned[5].try_borrow() {
                    if checkbox.is_checked() {
                        disable_defender_and_smartscreen::run()
                            .expect("disable_defender_and_smartscreen::run failed");
                    }
                }

                if let Ok(checkbox) = my_checkboxes_cloned[6].try_borrow() {
                    if checkbox.is_checked() {
                        disable_sleep::run().expect("disable_sleep::run failed");
                    }
                }

                if let Ok(checkbox) = my_checkboxes_cloned[7].try_borrow() {
                    if checkbox.is_checked() {
                        disable_vbs::run().expect("disable_vbs::run failed");
                    }
                }

                if let Ok(checkbox) = my_checkboxes_cloned[8].try_borrow() {
                    if checkbox.is_checked() {
                        defaults::run().expect("defaults::run failed");
                    }
                }

                if let Ok(checkbox) = my_checkboxes_cloned[9].try_borrow() {
                    if checkbox.is_checked() {
                        disable_recall::run().expect("disable_recall::run failed");
                    }
                }

                if my_checkboxes_cloned
                    .iter()
                    .all(|checkbox| checkbox.borrow().value() == false)
                {
                    dialog::message(
                        center().0,
                        center().1,
                        "No options were selected, therefore nothing has changed.",
                    );
                } else {
                    dialog::message(
                        center().0,
                        center().1,
                        "W11Boost applied your preferences successfully, please reboot.",
                    );
                }

                show_elements(
                    &frame0,
                    &apply_cloned,
                    &remove_cloned,
                    &my_checkboxes_cloned,
                );
            }
        });
    }

    {
        let frame0 = Rc::clone(&frame0);
        let apply_cloned = Rc::clone(&apply);
        let remove_cloned = Rc::clone(&remove);
        let my_checkboxes_cloned = my_checkboxes.clone();
        remove.borrow_mut().set_callback(move |_| {
            let choice = dialog::choice2(
                center().0,
                center().1,
                "Are you sure you want to remove W11Boost?",
                "&Yes",
                "&No",
                "",
            );
            if choice == Some(0) {
                hide_elements(
                    &frame0,
                    &apply_cloned,
                    &remove_cloned,
                    &my_checkboxes_cloned,
                );
                if let Ok(_) = remove_w11boost::run() {
                    dialog::message(
                        center().0,
                        center().1,
                        "W11Boost has been removed successfully, please reboot.",
                    );
                } else {
                    eprintln!("remove_w11boost::run failed");
                }

                show_elements(
                    &frame0,
                    &apply_cloned,
                    &remove_cloned,
                    &my_checkboxes_cloned,
                );
            }
        });
    }

    app.run().unwrap();

    Ok(())
}
