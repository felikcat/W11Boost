extern crate winresource;
use std::error::Error;
use std::path::{Path, PathBuf};

use winresource::WindowsResource;

// A workaround for my setup specifically
fn find_windows_sdk_bin() -> Option<PathBuf>
{
        let sdk_root = Path::new(r"C:\Program Files (x86)\Windows Kits\10\bin");
        if !sdk_root.exists() {
                return None;
        }

        // Find the latest SDK version
        let mut versions: Vec<_> = std::fs::read_dir(sdk_root)
                .ok()?
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_dir())
                .filter(|e| e.file_name().to_str().map(|s| s.starts_with("10.")).unwrap_or(false))
                .collect();

        versions.sort_by_key(|b| std::cmp::Reverse(b.file_name()));

        versions.first().map(|v| v.path().join("x64"))
}

fn main() -> Result<(), Box<dyn Error>>
{
        let mut res = WindowsResource::new();

        // Set Windows SDK path if found
        if let Some(sdk_path) = find_windows_sdk_bin() {
                res.set_toolkit_path(sdk_path.to_str().ok_or("Invalid SDK path string")?);
        }

        // Allowing for native DPI scaling and setting the Administrator requirement
        res.set_manifest(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly manifestVersion="1.0" xmlns="urn:schemas-microsoft-com:asm.v1" xmlns:asmv3="urn:schemas-microsoft-com:asm.v3">
  <trustInfo xmlns="urn:schemas-microsoft-com:asm.v2">
    <security>
      <requestedPrivileges xmlns="urn:schemas-microsoft-com:asm.v3">
        <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
      </requestedPrivileges>
    </security>
  </trustInfo>
  <asmv3:application>
    <asmv3:windowsSettings>
      <dpiAware xmlns="http://schemas.microsoft.com/SMI/2005/WindowsSettings">True/PM</dpiAware>
      <dpiAwareness xmlns="http://schemas.microsoft.com/SMI/2016/WindowsSettings">PerMonitorV2, PerMonitor</dpiAwareness>
    </asmv3:windowsSettings>
  </asmv3:application>
</assembly>"#,
    );
        res.compile()?;
        Ok(())
}
