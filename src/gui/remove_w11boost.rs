use crate::common::{restore_from_backup, run_system_command};
use anyhow::Result;
use std::{
        fs::{self, File},
        path::Path,
};
use winsafe::{co::FILE_ATTRIBUTE, SetFileAttributes};

pub fn run() -> Result<()>
{
        // Reverts all registry changes made by W11Boost.
        restore_from_backup()?;

        // Reset NTFS memory usage to default.
        run_system_command("fsutil.exe", &["behavior", "set", "memoryusage", "1"])?;

        // Re-enable last access timestamps.
        run_system_command("fsutil.exe", &["behavior", "set", "disablelastaccess", "2"])?;

        // Re-enable automatic recovery.
        run_system_command("bcdedit.exe", &["/set", "{default}", "recoveryenabled", "yes"])?;

        // Reset UserNotPresentSession.etl so Fast Startup can work again.
        let file_path = r"C:\Windows\System32\SleepStudy\UserNotPresentSession.etl";

        if Path::new(file_path).exists() {
                SetFileAttributes(file_path, FILE_ATTRIBUTE::NORMAL)?;
                fs::remove_file(file_path)?;
        }

        File::create(file_path)?;

        // Reset "Wait for Link" adapter setting (may silently fail on unsupported adapters).
        let _ = run_system_command(
                "powershell.exe",
                &[
                        "-Command",
                        "Get-NetAdapter | ForEach-Object { \
                                $prop = Get-NetAdapterAdvancedProperty -Name $_.Name -DisplayName 'Wait for Link' -ErrorAction SilentlyContinue; \
                                if ($prop) { \
                                        Reset-NetAdapterAdvancedProperty -Name $_.Name -DisplayName 'Wait for Link' -ErrorAction SilentlyContinue \
                                } \
                        }",
                ],
        );

        Ok(())
}
