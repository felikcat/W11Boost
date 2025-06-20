use crate::common::run_system_command;
use anyhow::Result;
use winsafe::{co::FILE_ATTRIBUTE, SetFileAttributes};
use std::{fs::{self, File}, path::Path};

pub fn run() -> Result<()>
{
        run_system_command("fsutil.exe", &["behavior", "set", "memoryusage", "1"])?;

        run_system_command("fsutil.exe", &["behavior", "set", "disablelastaccess", "2"])?;

        run_system_command("bcdedit.exe", &["/set", "{default}", "recoveryenabled", "yes"])?;

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
