use crate::common::*;
use std::error::Error;
use winsafe::{HKEY, prelude::advapi_Hkey};

pub fn run() -> Result<(), Box<dyn Error>> {
    let hklm = HKEY::LOCAL_MACHINE;

    // Don't allow online tips.
    set_dword(
        &hklm,
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer\AllowOnlineTips",
        "AllowOnlineTips",
        1,
    )?;

    // Don't allow users to enable online speech recongition services.
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\InputPersonalization",
        "AllowInputPersonalization",
        0,
    )?;

    // Remove Personalized Website Recommendations from the Recommended section in the Start Menu.
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
        "HideRecommendedPersonalizedSites",
        1,
    )?;

    // Prevent device metadata retrieval from the Internet.
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows\Device Metadata",
        "PreventDeviceMetadataFromNetwork",
        1,
    )?;

    // Turn off Search Companion content file updates.
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\SearchCompanion",
        "DisableContentFileUpdates",
        1,
    )?;

    // Don't allow Clipboard synchronization across devices.
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows\System",
        "AllowCrossDeviceClipboard",
        0,
    )?;

    Ok(())
}
