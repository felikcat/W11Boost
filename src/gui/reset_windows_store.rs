use std::{os::windows::process::CommandExt, process::Command};

use crate::common::CREATE_NO_WINDOW;

pub fn run() -> anyhow::Result<()>
{
        Command::new("wsreset.exe")
                .args(["-i"])
                .creation_flags(CREATE_NO_WINDOW)
                .output()?;

        Ok(())
}
