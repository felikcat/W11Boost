use windows::{core::w, Win32::System::{GroupPolicy::IGroupPolicyObject, Registry::{HKEY, HKEY_LOCAL_MACHINE}}};

use crate::common::*;
use std::error::Error;

/* Ignored for security or usability reasons:
    - Find My Device
    - Windows Update
    - Syncing to a Microsoft account
*/

pub fn run() -> Result<(), Box<dyn Error>> {
    let (hklm, gpo_hklm): (HKEY, IGroupPolicyObject) = init_registry_gpo(HKEY_LOCAL_MACHINE)?;

    // Don't allow online tips.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer\AllowOnlineTips"),
        w!("AllowOnlineTips"),
        1,
    )?;

    // Don't allow users to enable online speech recongition services.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\InputPersonalization"),
        w!("AllowInputPersonalization"),
        0,
    )?;

    // Remove Personalized Website Recommendations from the Recommended section in the Start Menu.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\Explorer"),
        w!("HideRecommendedPersonalizedSites"),
        1,
    )?;

    // Turn off account-based insights, recent, favorite, and recommended files in File Explorer.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\Explorer"),
        w!("HideRecommendedSection"),
        1,
    )?;

    // Prevent device metadata retrieval from the Internet.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\Device Metadata"),
        w!("PreventDeviceMetadataFromNetwork"),
        1,
    )?;

    // Turn off Search Companion content file updates.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\SearchCompanion"),
        w!("DisableContentFileUpdates"),
        1,
    )?;

    // Don't allow Clipboard synchronization across devices.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\System"),
        w!("AllowCrossDeviceClipboard"),
        0,
    )?;

    // Don't allow downloading updates to the Disk Failure Prediction Model.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\StorageHealth"),
        w!("AllowDiskHealthModelUpdates"),
        0,
    )?;

    // Don't allow sideloaded apps to auto-update in the background.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\Appx"),
        w!("DisableBackgroundAutoUpdates"),
        0,
    )?;

    // "Cloud optimized content / Windows experiences" are used for advertising, but aren't disabled in defaults.rs to keep the OS "stock".
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\CloudContent"),
        w!("DisableCloudOptimizedContent"),
        1,
    )?;
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\CloudContent"),
        w!("DisableConsumerAccountStateContent"),
        1,
    )?;
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\CloudContent"),
        w!("DisableSoftLanding"),
        1,
    )?;
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\CloudContent"),
        w!("DisableSoftLanding"),
        1,
    )?;
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\CloudContent"),
        w!("DisableWindowsConsumerFeatures"),
        1,
    )?;

    // Don't allow Windows to sync cellular messages to Microsoft's cloud services.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\Messaging"),
        w!("AllowMessageSync"),
        0,
    )?;

    // Disable an old virtual assistant that excessively used the internet and violated privacy.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\Windows Search"),
        w!("AllowCortana"),
        0,
    )?;

    // Disable Windows Search from using the "cloud" / internet.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\Windows Search"),
        w!("AllowCloudSearch"),
        0,
    )?;
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\Windows Search"),
        w!("DisableWebSearch"),
        1,
    )?;
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\Windows Search"),
        w!("EnableDynamicContentInWSB"),
        1,
    )?;
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\Windows Search"),
        w!("ConnectedSearchUseWeb"),
        0,
    )?;

    // Don't automatically download a new speech model.
    set_dword_gpo(
        hklm,
        w!(r"Machine\SOFTWARE\Policies\Microsoft\Speech"),
        w!("AllowSpeechModelUpdate"),
        0,
    )?;

    // Don't show News and Interests or other widgets.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Dsh"),
        w!("AllowNewsAndInterests"),
        0,
    )?;

    save_registry_gpo(hklm, gpo_hklm)?;
    
    Ok(())
}
