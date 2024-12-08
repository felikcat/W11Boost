mod appx_support;
mod create_system_restore_point;
mod defaults;
mod disable_defender_and_smartscreen;
mod disable_sleep;
mod disable_vbs;
mod reduce_forensics;
mod reduce_online_data_collection;
mod remove_w11boost;

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
    borrow::BorrowMut,
    cell::RefCell,
    error::Error,
    mem,
    ops::DerefMut,
    process::{Command, exit},
    rc::Rc,
};
use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{
        HWND_TOPMOST, SWP_FRAMECHANGED, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW, SetWindowPos,
    },
};

use crate::common::center;
type MyCheckboxes = [CheckButton; 9];

pub fn draw_gui() -> Result<(), Box<dyn Error>> {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let font = app.load_font("C:\\Windows\\Fonts\\segoeui.ttf").unwrap();

    let mut wind = Window::default()
        .with_label("W11Boost")
        .with_size(480, 480)
        .center_screen();

    wind.set_border(false);

    let widget_theme = ColorTheme::new(color_themes::BLACK_THEME);
    widget_theme.apply();

    let mut titlebar = frame::Frame::new(0, 0, 480, 32, None);
    titlebar.set_frame(enums::FrameType::FlatBox);
    titlebar.set_color(Color::from_rgb(22, 22, 22));

    let mut titlebar_close = Button::new(wind.width() - 32, 0, 32, 32, "X");

    titlebar_close.set_frame(enums::FrameType::NoBox);
    titlebar_close.set_callback(move |_| exit(0));

    let mut apply = Button::new(
        0,
        0,
        (wind.width() - 6) / 2,
        (wind.height() * 14) / 100,
        "Apply W11Boost",
    )
    .center_of(&wind);

    let mut remove = Button::new(
        wind.width() / 2,
        0,
        (wind.width() - 6) / 2,
        (wind.height() * 14) / 100,
        "Remove W11Boost",
    )
    .center_of(&wind);

    apply.set_pos(2, wind.height() - apply.height() - 2); // Put button at the bottom

    remove.set_pos(
        wind.width() - remove.width() - 2,
        wind.height() - remove.height() - 2,
    );

    apply.set_label_font(enums::Font::by_name(&font));
    apply.set_label_size(16);
    remove.set_label_font(enums::Font::by_name(&font));
    remove.set_label_size(16);

    let checkbox_height = wind.height() / 12;

    let mut my_checkboxes: MyCheckboxes = [
        CheckButton::new(
            0,
            titlebar.height(),
            wind.width(),
            checkbox_height,
            "Reduce local data / forensics to a minimum",
        ),
        CheckButton::new(
            0,
            titlebar.height() + checkbox_height + 2,
            wind.width(),
            checkbox_height,
            "Reduce online activity to a minimum",
        ),
        CheckButton::new(
            0,
            titlebar.height() + checkbox_height * 2 + 4,
            wind.width(),
            checkbox_height,
            "Create a system restore point",
        ),
        CheckButton::new(
            0,
            titlebar.height() + checkbox_height * 3 + 6,
            wind.width(),
            checkbox_height,
            "Install the Microsoft Store",
        ),
        CheckButton::new(
            0,
            titlebar.height() + checkbox_height * 4 + 8,
            wind.width(),
            checkbox_height,
            "Install support for .appx/.appxbundle and WinGet",
        ),
        CheckButton::new(
            0,
            titlebar.height() + checkbox_height * 5 + 10,
            wind.width(),
            checkbox_height,
            "Disable Defender and SmartScreen",
        ),
        CheckButton::new(
            0,
            titlebar.height() + checkbox_height * 6 + 12,
            wind.width(),
            checkbox_height,
            "Disable sleep and hibernate",
        ),
        CheckButton::new(
            0,
            titlebar.height() + checkbox_height * 7 + 14,
            wind.width(),
            checkbox_height,
            "Disable Virtualization Based Security",
        ),
        CheckButton::new(
            0,
            titlebar.height() + checkbox_height * 8 + 16,
            wind.width(),
            checkbox_height,
            "Add in non-intrusive tweaks",
        ),
    ];

    for checkbox in &mut my_checkboxes {
        checkbox.set_label_font(enums::Font::by_name(&font));
        checkbox.set_label_size(16);
    }

    my_checkboxes[2].set_value(true);
    my_checkboxes[8].set_value(true);

    let frame0 = RefCell::new(Widget::default()
        .with_size(wind.width(), wind.height() - titlebar.height())
        .with_pos(0, titlebar.height()));

    let frame0_clone = frame0.clone();

    frame0_clone.borrow_mut().set_frame(enums::FrameType::BorderBox);
    frame0_clone.borrow_mut().draw(move |f| {
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
    frame0_clone.borrow_mut().set_label("Applying W11Boost, please wait...");
    frame0_clone.borrow_mut().hide();

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
        frame0: &mut impl WidgetExt,
        apply_clone: &mut impl ButtonExt,
        my_checkboxes: &mut [impl WidgetExt],
    ) {
        frame0.show();
        frame0.top_window();

        // So these aren't accidentally clicked.
        apply_clone.hide();
        for checkbox in my_checkboxes {
            checkbox.hide();
        }

        // Force window to redraw to display frame0.
        app::flush();
        app::wait();
    }

    fn show_elements(
        frame0: &mut impl WidgetExt,
        apply_clone: &mut impl ButtonExt,
        my_checkboxes: &mut [impl WidgetExt],
    ) {
        // Does not require a manual redraw.
        frame0.hide();
        apply_clone.show();
        for checkbox in my_checkboxes {
            checkbox.show();
        }
    }

    let mut apply_clone = apply.clone();

    apply.set_callback( move |_| {
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
                &mut *frame0_clone.borrow_mut(),
                &mut *apply_clone.borrow_mut(),
                &mut *my_checkboxes.borrow_mut(),
            );
            // Has to be first!
            if my_checkboxes[2].is_checked() {
                create_system_restore_point::run()
                    .expect("create_system_restore_point::run failed");
            }

            if my_checkboxes[0].is_checked() {
                reduce_forensics::run().expect("reduce_forensics::run failed");
            }
            if my_checkboxes[1].is_checked() {
                reduce_online_data_collection::run()
                    .expect("reduce_online_data_collection::run failed");
            }
            if my_checkboxes[3].is_checked() {
                Command::new("wsreset.exe")
                    .output()
                    .expect("wsreset.exe failed to execute");
            }
            if my_checkboxes[4].is_checked() {
                appx_support::install().expect("appx_support::install failed");
            }

            if my_checkboxes[5].is_checked() {
                disable_defender_and_smartscreen::run()
                    .expect("disable_defender_and_smartscreen::run failed");
            }

            if my_checkboxes[6].is_checked() {
                disable_sleep::run().expect("disable_sleep::run failed");
            }

            if my_checkboxes[7].is_checked() {
                disable_vbs::run().expect("disable_vbs::run failed");
            }

            if my_checkboxes[8].is_checked() {
                defaults::run().expect("defaults::run failed");
            }

            if my_checkboxes.iter().all(|checkbox| !checkbox.is_checked()) {
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
                &mut *frame0_clone.borrow_mut(),
                &mut *apply_clone.borrow_mut(),
                &mut *my_checkboxes.borrow_mut(),
            );
        }
    });

    remove.set_callback(move |_| {
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
                &mut *frame0_clone.borrow_mut(),
                &mut apply_clone,
                &mut my_checkboxes,
            );
            if let Ok(_) = remove_w11boost::run() {
                dialog::message(
                    center().0,
                    center().1,
                    "W11Boost applied your preferences successfully, please reboot.",
                );
            } else {
                eprintln!("remove_w11boost::run failed");
            }

            show_elements(
                &mut *frame0_clone.borrow_mut(),
                &mut apply_clone,
                &mut my_checkboxes,
            );
        }
    });

    app.run().unwrap();
    Ok(())
}
