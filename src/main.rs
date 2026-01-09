#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![deny(
        clippy::cargo,
        clippy::complexity,
        clippy::correctness,
        clippy::perf,
        clippy::style,
        clippy::suspicious
)]
#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
        clippy::too_many_lines,
        clippy::missing_errors_doc,
        clippy::multiple_crate_versions,
        clippy::single_call_fn,
        clippy::implicit_return,
        clippy::missing_docs_in_private_items,
        clippy::min_ident_chars,
        clippy::arbitrary_source_item_ordering,
        clippy::question_mark_used
)]

use w11boost::gui;

use std::panic;
use windows::Win32::UI::WindowsAndMessaging::{MB_ICONERROR, MB_OK, MessageBoxW};
use windows::core::PCWSTR;

fn show_error_box(title: &str, message: &str)
{
        let msg_wide: Vec<u16> = message.encode_utf16().chain(std::iter::once(0)).collect();
        let title_wide: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
        unsafe {
                MessageBoxW(
                        None,
                        PCWSTR(msg_wide.as_ptr()),
                        PCWSTR(title_wide.as_ptr()),
                        MB_OK | MB_ICONERROR,
                );
        }
}

fn main()
{
        // Set up panic handler to show message box
        panic::set_hook(Box::new(|info| {
                let location = info
                        .location()
                        .map_or_else(|| "unknown".to_string(), std::string::ToString::to_string);

                let payload = info.payload();
                let msg = if let Some(s) = payload.downcast_ref::<&str>() {
                        format!("Panic: {s}\n\nLocation: {location}")
                } else if let Some(s) = payload.downcast_ref::<String>() {
                        format!("Panic: {s}\n\nLocation: {location}")
                } else {
                        format!("Panic occurred\n\nLocation: {location}")
                };

                show_error_box("W11Boost Panic", &msg);
        }));

        if let Err(e) = gui::run_gui() {
                show_error_box("W11Boost Error", &format!("W11Boost -> run_gui() failed.\nError: {e}"));
        }
}
