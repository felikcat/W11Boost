#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![forbid(unsafe_code)]
#![deny(
        clippy::cargo,
        clippy::complexity,
        clippy::correctness,
        clippy::perf,
        clippy::style,
        clippy::suspicious,
)]
#![warn(
        clippy::pedantic,
        clippy::nursery,
)]
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

pub mod common;
mod gui;
use common::center;
use fltk::dialog;
use gui::draw_gui;

fn main()
{
        match draw_gui() {
                Ok(()) => println!("draw_gui() exited successfully"),
                Err(e) => dialog::alert(
                        center().0,
                        center().1,
                        &format!("W11Boost -> draw_gui() failed.\nError:{e}"),
                ),
        }
}
