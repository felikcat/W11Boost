use crate::common::CREATE_NO_WINDOW;
use anyhow::Result;
use winsafe::{co::FILE_ATTRIBUTE, SetFileAttributes};
use std::{fs::{self, File}, os::windows::process::CommandExt, path::Path, process::Command};

pub fn run() -> Result<()>
{
        Command::new("fsutil.exe")
                .args(["behavior", "set", "memoryusage", "1"])
                .creation_flags(CREATE_NO_WINDOW)
                .output()?;

        Command::new("fsutil.exe")
                .args(["behavior", "set", "disablelastaccess", "2"])
                .creation_flags(CREATE_NO_WINDOW)
                .output()?;

        Command::new("bcdedit.exe")
                .args(["/set", "{default}", "recoveryenabled", "yes"])
                .creation_flags(CREATE_NO_WINDOW)
                .output()?;

        let file_path = r"C:\Windows\System32\SleepStudy\UserNotPresentSession.etl";

        if Path::new(file_path).exists() {
                // Removing UserNotPresentSession.etl will fail if it is readonly.
                SetFileAttributes(file_path, FILE_ATTRIBUTE::NORMAL)?;
                fs::remove_file(file_path)?;
        }

        // Re-create UserNotPresentSession.etl with NORMAL file attributes.
        File::create(file_path)?;

        Ok(())
}
