use crate::common::*;
use std::error::Error;
use winsafe::{HKEY, prelude::advapi_Hkey};

/* Ignored:
    - Find My Device
    - Windows Update
*/

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

    // Turn off account-based insights, recent, favorite, and recommended files in File Explorer.
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
        "HideRecommendedSection",
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

    // Don't allow downloading updates to the Disk Failure Prediction Model.
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows\StorageHealth",
        "AllowDiskHealthModelUpdates",
        0,
    )?;

    // Don't allow sideloaded apps to auto-update in the background.
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows\Appx",
        "DisableBackgroundAutoUpdates",
        0
    )?;

    // "Cloud optimized content / Windows experiences" are used for advertising, but aren't disabled in defaults.rs to keep the OS "stock".
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
        "DisableCloudOptimizedContent",
        1
    )?;
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
        "DisableConsumerAccountStateContent",
        1
    )?;
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
        "DisableSoftLanding",
        1
    )?;
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
        "DisableSoftLanding",
        1
    )?;
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows\CloudContent",
        "DisableWindowsConsumerFeatures",
        1
    )?;

    // Don't allow Windows to sync cellular messages to Mircosoft's cloud services.
    set_dword(&hklm, r"SOFTWARE\Policies\Microsoft\Windows\Messaging", "AllowMessageSync", 0)?;

    // Disable an old virtual assistant that excessively used the internet and violated privacy.
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
        "AllowCortana",
        0
    )?;

    // Disable Windows Search from using the "cloud" / internet.
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
        "AllowCloudSearch",
        0
    )?;
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
        "DisableWebSearch",
        1
    )?;
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
        "EnableDynamicContentInWSB",
        1
    )?;
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
        "ConnectedSearchUseWeb",
        0
    )?;


    Ok(())
}
