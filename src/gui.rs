pub mod appx_support;
mod create_system_restore_point;
mod defaults;
mod disable_sleep;
mod reduce_local_data_collection;
mod reduce_online_data_collection;

use fltk::{
    app::{self, Screen}, button::{Button, CheckButton}, draw::{self}, enums::{self, Color}, frame::{self}, prelude::*, widget::Widget, window::Window
};
use fltk_theme::{ColorTheme, color_themes};
use std::{
    error::Error,
    mem,
    process::{Command, exit},
};
use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{
        HWND_TOPMOST, SWP_FRAMECHANGED, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW, SetWindowPos,
    },
};
type MyCheckboxes = [CheckButton; 6];

pub fn draw_gui() -> Result<(), Box<dyn Error>> {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let font = app.load_font("C:\\Windows\\Fonts\\segoeui.ttf").unwrap();

    let mut wind = Window::default()
        .with_label("W11Boost")
        .with_size(480, 360)
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
        wind.width() - 2,
        wind.height() / 10 * 2,
        "Apply W11Boost",
    )
    .center_of(&wind);

    apply.set_pos(
        wind.width() - apply.width() - 1,
        wind.height() - apply.height() - 2,
    ); // Put button at the bottom

    apply.set_label_font(enums::Font::by_name(&font));
    apply.set_label_size(16);

    let checkbox_height = wind.height() / 10;

    let mut my_checkboxes: MyCheckboxes = [
        CheckButton::new(
            0,
            titlebar.height(),
            wind.width(),
            checkbox_height,
            "Reduce local data collection",
        ),
        CheckButton::new(
            0,
            titlebar.height() + wind.height() / 10 + 2,
            wind.width(),
            checkbox_height,
            "Reduce online data collection",
        ),
        CheckButton::new(
            0,
            titlebar.height() + wind.height() / 10 * 2 + 4,
            wind.width(),
            checkbox_height,
            "Create a system restore point",
        ),
        CheckButton::new(
            0,
            titlebar.height() + wind.height() / 10 * 3 + 6,
            wind.width(),
            checkbox_height,
            "Install the Microsoft Store",
        ),
        CheckButton::new(
            0,
            titlebar.height() + wind.height() / 10 * 4 + 8,
            wind.width(),
            checkbox_height,
            "Install support for .appx/.appxbundle and WinGet",
        ),
        CheckButton::new(
            0,
            titlebar.height() + wind.height() / 10 * 5 + 10,
            wind.width(),
            checkbox_height,
            "Disable sleep and hibernate",
        ),
    ];

    for checkbox in &mut my_checkboxes {
        checkbox.set_label_font(enums::Font::by_name(&font));
        checkbox.set_label_size(16);
    }

    my_checkboxes[2].set_value(true);

    let mut frame0 = Widget::default()
        .with_size(wind.width(), wind.height() - titlebar.height())
        .with_pos(0, titlebar.height());
    frame0.set_frame(enums::FrameType::BorderBox);
    frame0.draw( move |f| {
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
    frame0.set_label("Applying W11Boost, please wait...");
    frame0.hide();

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
        480,
        360,
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

    apply.set_callback(move |_| {
        frame0.show();
        frame0.top_window();

        // Force window to redraw to display frame0.
        app::flush();
        app::wait();
        
        // Has to be first!
        if my_checkboxes[2].is_checked() {
            create_system_restore_point::run().expect("create_system_restore_point::run failed");
        }
        if my_checkboxes[0].is_checked() {
            reduce_local_data_collection::run().expect("reduce_local_data_collection::run failed");
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
            disable_sleep::run().expect("disable_sleep::run failed");
        }

        defaults::run().expect("defaults::run failed");

        // Does not require a manual redraw.
        frame0.hide();
    });

    app.run().unwrap();
    Ok(())
}
