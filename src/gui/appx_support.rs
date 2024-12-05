use curl::easy::Easy;
use winsafe::co::KNOWNFOLDERID;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;
use crate::common::*;

pub fn install() -> Result<(), Box<dyn Error>> {
    let desktop_dir = get_windows_path(&KNOWNFOLDERID::PublicDesktop)?;
    let mut path = PathBuf::from(desktop_dir);

    let mut easy = Easy::new();

    easy.url("https://github.com/microsoft/winget-cli/releases/latest/download/Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle")?;

    easy.useragent("Mozilla/5.0 (Windows NT 10.0; WOW64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.6556.192 Safari/537.36")?;

    easy.follow_location(true)?;
    easy.connect_timeout(Duration::from_secs(10))?;

    path.push("Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle");
    let mut file =
        File::create(Path::new(&path)).expect("appx_support::install -> Failed to create file");

    easy.write_function(move |data| {
        file.write_all(data).unwrap();
        Ok(data.len())
    })
    .expect("appx_support::install -> Failed to write data");

    easy.perform()
        .expect("appx_support::install -> Failed to curl perform");

    Command::new("powershell.exe")
        .args([
            "-Command",
            r#"Add-AppxPackage ([Environment]::GetFolderPath("CommonDesktopDirectory") + "\Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle""#
        ])
        .output()
        .expect("appx_support::install -> Failed to install the msixbundle");

    Ok(())
}
