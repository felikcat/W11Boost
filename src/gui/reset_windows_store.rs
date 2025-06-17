use std::{os::windows::process::CommandExt, process::Command};
use anyhow::Result;

use crate::common::CREATE_NO_WINDOW;

pub fn run() -> Result<()>
{
        Command::new("wsreset.exe")
                .args(["-i"])
                .creation_flags(CREATE_NO_WINDOW)
                .output()?;

        Ok(())
}
