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
    borrow::{Borrow, BorrowMut},
    cell::{Ref, RefCell},
    error::Error,
    mem,
    ops::DerefMut,
    process::{exit, Command},
    rc::Rc,
};
use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{
        HWND_TOPMOST, SWP_FRAMECHANGED, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW, SetWindowPos,
    },
};

use crate::common::center;
type MyCheckboxes = [RefCell<CheckButton>; 9];

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

    let apply = Rc::new(RefCell::new(Button::new(
        0,
        0,
        (wind.width() - 6) / 2,
        (wind.height() * 14) / 100,
        "Apply W11Boost",
    ).center_of(&wind)));
    
    let apply = Rc::clone(&apply);

    let mut apply_mut = (*apply).borrow_mut();

    let apply_height = apply_mut.clone().height();
    apply_mut.set_pos(2, wind.height() - apply_height - 2); // Put button at the bottom

    let remove = Rc::new(RefCell::new(Button::new(
        wind.width() / 2,
        0,
        (wind.width() - 6) / 2,
        (wind.height() * 14) / 100,
        "Remove W11Boost",
    )));
    
    let remove = Rc::clone(&remove);
    let mut remove_mut = (*remove).borrow_mut();

    remove_mut.clone().center_of(&wind);

    let remove_width = remove_mut.width();
    let remove_height = remove_mut.height();
    remove_mut.set_pos(
        wind.width() - remove_width - 2,
        wind.height() - remove_height - 2,
    );

    remove_mut.set_label_font(enums::Font::by_name(&font));
    remove_mut.set_label_size(16);

    apply_mut.set_label_font(enums::Font::by_name(&font));
    apply_mut.set_label_size(16);

    let checkbox_height = wind.height() / 12;

    let mut my_checkboxes: MyCheckboxes = [
        RefCell::new(CheckButton::new(
            0,
            titlebar.height(),
            wind.width(),
            checkbox_height,
            "Reduce local data / forensics to a minimum",
        )),
        RefCell::new(CheckButton::new(
            0,
            titlebar.height() + checkbox_height + 2,
            wind.width(),
            checkbox_height,
            "Reduce online activity to a minimum",
        )),
        RefCell::new(CheckButton::new(
            0,
            titlebar.height() + checkbox_height * 2 + 4,
            wind.width(),
            checkbox_height,
            "Create a system restore point",
        )),
        RefCell::new(CheckButton::new(
            0,
            titlebar.height() + checkbox_height * 3 + 6,
            wind.width(),
            checkbox_height,
            "Install the Microsoft Store",
        )),
        RefCell::new(CheckButton::new(
            0,
            titlebar.height() + checkbox_height * 4 + 8,
            wind.width(),
            checkbox_height,
            "Install support for .appx/.appxbundle and WinGet",
        )),
        RefCell::new(CheckButton::new(
            0,
            titlebar.height() + checkbox_height * 5 + 10,
            wind.width(),
            checkbox_height,
            "Disable Defender and SmartScreen",
        )),
        RefCell::new(CheckButton::new(
            0,
            titlebar.height() + checkbox_height * 6 + 12,
            wind.width(),
            checkbox_height,
            "Disable sleep and hibernate",
        )),
        RefCell::new(CheckButton::new(
            0,
            titlebar.height() + checkbox_height * 7 + 14,
            wind.width(),
            checkbox_height,
            "Disable Virtualization Based Security",
        )),
        RefCell::new(CheckButton::new(
            0,
            titlebar.height() + checkbox_height * 8 + 16,
            wind.width(),
            checkbox_height,
            "Add in non-intrusive tweaks",
        )),
    ];

    let my_checkboxes = my_checkboxes.clone();

    for checkbox in &my_checkboxes {
        let mut checkbox = checkbox.borrow_mut();
        checkbox.set_label_font(enums::Font::by_name(&font));
        checkbox.set_label_size(16);
    }

    my_checkboxes[2].borrow_mut().set_value(true);
    my_checkboxes[8].borrow_mut().set_value(true);

    let frame0 = RefCell::new(
        Widget::default()
            .with_size(wind.width(), wind.height() - titlebar.height())
            .with_pos(0, titlebar.height()),
    );

    let frame0 = frame0.clone();

    frame0
        .borrow_mut()
        .set_frame(enums::FrameType::BorderBox);
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
        apply: &mut impl ButtonExt,
        remove: &mut impl ButtonExt,
        my_checkboxes: &[RefCell<CheckButton>],
    ) {
        frame0.show();
        frame0.top_window();

        // So these aren't accidentally clicked.
        apply.hide();
        remove.hide();
        for checkbox in my_checkboxes {
            checkbox.borrow_mut().hide();
        }

        // Force window to redraw to display frame0.
        app::flush();
        app::wait();
    }

    fn show_elements(
        frame0: &mut impl WidgetExt,
        apply: &mut impl ButtonExt,
        remove: &mut impl ButtonExt,
        my_checkboxes: &[RefCell<CheckButton>],
    ) {
        // Does not require a manual redraw.
        frame0.hide();
        apply.show();
        remove.show();
        for checkbox in my_checkboxes {
            checkbox.borrow_mut().show();
        }
    }
    (*apply_mut).set_callback({
        let frame0 = frame0.clone();
        let apply = apply.clone();
        let remove = remove.clone();
        let my_checkboxes = my_checkboxes.clone();
        move |_| {
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
                    &mut *frame0.borrow_mut(),
                    &mut *(*apply).borrow_mut(),
                    &mut *(*remove).borrow_mut(),
                    &my_checkboxes,
                );
                // Has to be first!
                if let Ok(checkbox) = my_checkboxes[2].try_borrow_mut() {
                    if checkbox.is_checked() {
                        create_system_restore_point::run()
                            .expect("create_system_restore_point::run failed");
                    }
                }

                if let Ok(checkbox) = my_checkboxes[0].try_borrow_mut() {
                    if checkbox.is_checked() {
                        reduce_forensics::run().expect("reduce_forensics::run failed");
                    }
                }
                if let Ok(checkbox) = my_checkboxes[1].try_borrow_mut() {
                    if checkbox.is_checked() {
                        reduce_online_data_collection::run()
                            .expect("reduce_online_data_collection::run failed");
                    }
                }
                if let Ok(checkbox) = my_checkboxes[3].try_borrow_mut() {
                    if checkbox.is_checked() {
                        Command::new("wsreset.exe")
                            .output()
                            .expect("wsreset.exe failed to execute");
                    }
                }
                if let Ok(checkbox) = my_checkboxes[4].try_borrow_mut() {
                    if checkbox.is_checked() {
                        appx_support::install().expect("appx_support::install failed");
                    }
                }

                if let Ok(checkbox) = my_checkboxes[5].try_borrow_mut() {
                    if checkbox.is_checked() {
                        disable_defender_and_smartscreen::run()
                            .expect("disable_defender_and_smartscreen::run failed");
                    }
                }

                if let Ok(mut checkbox) = my_checkboxes[6].try_borrow_mut() {
                    if checkbox.is_checked() {
                        disable_sleep::run().expect("disable_sleep::run failed");
                    }
                }

                if let Ok(mut checkbox) = my_checkboxes[7].try_borrow_mut() {
                    if checkbox.is_checked() {
                        disable_vbs::run().expect("disable_vbs::run failed");
                    }
                }

                if let Ok(mut checkbox) = my_checkboxes[8].try_borrow_mut() {
                    if checkbox.is_checked() {
                        defaults::run().expect("defaults::run failed");
                    }
                }

                if my_checkboxes.iter().all(|checkbox| checkbox.try_borrow_mut().map_or(false, |mut cb| !cb.is_checked())) {
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
                    &mut *frame0.borrow_mut(),
                    &mut *(*apply).borrow_mut(),
                    &mut *(*remove).borrow_mut(),
                    &my_checkboxes,
                );
            }
        }
    });

    (*remove_mut).borrow_mut().set_callback({
        let frame0 = frame0.clone();
        let apply = apply.clone();
        let remove = remove.clone();
        let my_checkboxes = my_checkboxes.clone();
        move |_| {
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
                    &mut *frame0.borrow_mut(),
                    &mut *(*apply).borrow_mut(),
                    &mut *(*remove).borrow_mut(),
                    &my_checkboxes,
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
                    &mut *frame0.borrow_mut(),
                    &mut *(*apply).borrow_mut(),
                    &mut *(*remove).borrow_mut(),
                    &my_checkboxes,
                );
            }
        }
    });

    app.run().unwrap();
    Ok(())
}
