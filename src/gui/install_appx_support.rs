use crate::common::{run_system_command, get_windows_path};
use anyhow::Result;
use core::time::Duration;
use curl::easy::Easy;
use std::fs::File;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use winsafe::co::KNOWNFOLDERID;

pub fn run() -> Result<()>
{
        let desktop_dir = get_windows_path(&KNOWNFOLDERID::Desktop)?;
        let mut path = PathBuf::from(desktop_dir);

        let mut easy = Easy::new();

        easy.url("https://github.com/microsoft/winget-cli/releases/latest/download/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle")?;

        easy.useragent("Mozilla/5.0 (Windows NT 11.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.6998.166 Safari/537.36")?;

        easy.follow_location(true)?;
        easy.connect_timeout(Duration::from_secs(10))?;

        path.push("Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle");
        let mut file = File::create(Path::new(&path))?;

        easy.write_function(move |data| {
                file.write_all(data).unwrap();
                Ok(data.len())
        })?;

        easy.perform()?;

        run_system_command(
                "powershell.exe",
                &[
                        "-Command",
                        r#"Add-AppxPackage ([Environment]::GetFolderPath("DesktopDirectory") + "\Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle")"#,
                ],
        )?;

        Ok(())
}
