use anyhow::Result;
use winsafe::HKEY;

use w11boost::set_dword;

/* Ignored for security or usability reasons:
    - Find My Device
    - Windows Update
    - Syncing to a Microsoft account
*/

// (hkey: "HKLM"|"HKCU", subkey, value_name, value)
const REGISTRY_DWORDS: &[(&str, &str, &str, u32)] = &[
	// Don't allow online tips.
	("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer\AllowOnlineTips", "AllowOnlineTips", 1),
	// Don't allow users to enable online speech recognition services.
	("HKLM", r"SOFTWARE\Policies\Microsoft\InputPersonalization", "AllowInputPersonalization", 0),
	// Remove Personalized Website Recommendations from the Recommended section in Start Menu.
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "HideRecommendedPersonalizedSites", 1),
	// Turn off account-based insights, recent, favorite, and recommended files in File Explorer.
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "HideRecommendedSection", 1),
	// Prevent device metadata retrieval from the Internet.
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Device Metadata", "PreventDeviceMetadataFromNetwork", 1),
	// Turn off Search Companion content file updates.
	("HKLM", r"SOFTWARE\Policies\Microsoft\SearchCompanion", "DisableContentFileUpdates", 1),
	// Don't allow Clipboard synchronization across devices.
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "AllowCrossDeviceClipboard", 0),
	// Don't allow downloading updates to the Disk Failure Prediction Model.
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\StorageHealth", "AllowDiskHealthModelUpdates", 0),
	// Don't allow sideloaded apps to auto-update in the background.
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Appx", "DisableBackgroundAutoUpdates", 0),
	// "Cloud optimized content / Windows experiences" are used for advertising.
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\CloudContent", "DisableCloudOptimizedContent", 1),
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\CloudContent", "DisableConsumerAccountStateContent", 1),
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\CloudContent", "DisableSoftLanding", 1),
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\CloudContent", "DisableWindowsConsumerFeatures", 1),
	// Don't allow Windows to sync cellular messages to Microsoft's cloud services.
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Messaging", "AllowMessageSync", 0),
	// Disable Cortana.
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Search", "AllowCortana", 0),
	// Disable Windows Search from using the "cloud" / internet.
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Search", "AllowCloudSearch", 0),
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Search", "DisableWebSearch", 1),
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Search", "EnableDynamicContentInWSB", 1),
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Search", "ConnectedSearchUseWeb", 0),
	// Don't automatically download a new speech model.
	("HKLM", r"SOFTWARE\Policies\Microsoft\Speech", "AllowSpeechModelUpdate", 0),
	// Don't show News and Interests or other widgets.
	("HKLM", r"SOFTWARE\Policies\Microsoft\Dsh", "AllowNewsAndInterests", 0),
	// Disable an extra lockscreen that is mainly served to put widgets on.
	("HKLM", r"Software\Policies\Microsoft\Windows\Personalization", "NoLockScreen", 1),
];

fn get_hkey(hkey_str: &str) -> HKEY
{
	if hkey_str == "HKLM" { HKEY::LOCAL_MACHINE } else { HKEY::CURRENT_USER }
}

pub fn run() -> Result<()>
{
	for (hkey_str, subkey, value_name, value) in REGISTRY_DWORDS {
		set_dword(&get_hkey(hkey_str), subkey, value_name, *value)?;
	}
	Ok(())
}
