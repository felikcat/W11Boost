#![windows_subsystem = "windows"]
#![forbid(unsafe_code)]

pub mod common;
mod gui;
use common::center;
use fltk::dialog;
use gui::draw_gui;

fn main()
{
        match draw_gui() {
                Ok(_) => println!("draw_gui() exited successfully"),
                Err(e) => dialog::alert(
                        center().0,
                        center().1,
                        &format!("W11Boost -> draw_gui() failed.\nError:{e}"),
                ),
        }
}
