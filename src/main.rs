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

mod gui;
use gui::draw_gui;
use winsafe::prelude::*;
use winsafe::{HWND, co::MB};

fn main()
{
        match draw_gui() {
                Ok(()) => {}
                Err(e) => {
                        let msg = format!("W11Boost -> draw_gui() failed.\nError: {e}");
                        let _ = HWND::NULL.MessageBox(&msg, "W11Boost Error", MB::OK | MB::ICONERROR);
                }
        }
}
