use std::{error::Error, os::windows::process::CommandExt, process::Command};

use crate::common::CREATE_NO_WINDOW;

pub fn run() -> Result<(), Box<dyn Error>> {
        Command::new("wsreset.exe")
                .args(["-i"])
                .creation_flags(CREATE_NO_WINDOW)
                .output()?;

        Ok(())
}
