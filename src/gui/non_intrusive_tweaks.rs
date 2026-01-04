use std::fs::{self, File};
use std::path::Path;

use anyhow::Result;
use winsafe::co::FILE_ATTRIBUTE;
use winsafe::{HKEY, SetFileAttributes};

use w11boost::{remove_subkey, run_system_command, set_dword, set_string};

const USER_NOT_PRESENT_SESSION_PATH: &str = r"C:\Windows\System32\SleepStudy\UserNotPresentSession.etl";

// (hkey: "HKLM"|"HKCU", subkey, value_name, value)
const REGISTRY_DWORDS: &[(&str, &str, &str, u32)] = &[
	// If allowed (1): unused apps would be uninstalled with their user data left intact.
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Appx", "AllowAutomaticAppArchiving", 0),
	// Make all users opted out of the Windows Customer Experience Improvement Program.
	("HKLM", r"SOFTWARE\Policies\Microsoft\SQMClient\Windows", "CEIPEnable", 0),
	// Shows what's slowing down bootups and shutdowns.
	("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System", "verbosestatus", 1),
	// Ask to not allow execution of experiments by Microsoft.
	("HKLM", r"SOFTWARE\Microsoft\PolicyManager\current\device\System", "AllowExperimentation", 0),
	// Power Throttling causes severe performance reduction for VMWare Workstation 17.
	("HKLM", r"SYSTEM\CurrentControlSet\Control\Power\PowerThrottling", "PowerThrottlingOff", 1),
	// FTH being enabled causes issues with specific apps such as Assetto Corsa.
	("HKLM", r"SOFTWARE\Microsoft\FTH", "Enabled", 0),
	// Automated file cleanup without user interaction is a bad idea.
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Appx", "AllowStorageSenseGlobal", 0),
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\StorageSense", "AllowStorageSenseGlobal", 0),
	// Allocate more RAM to NTFS' paged pool.
	("HKLM", r"SYSTEM\CurrentControlSet\Policies", "NtfsForceNonPagedPoolAllocation", 1),
	// Do not page drivers and other system code to a disk, keep it in memory.
	("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "DisablePagingExecutive", 1),
	// Disables "Fast startup", a form of hibernation.
	("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Power", "HiberbootEnabled", 0),
	// Do not require use of fast startup.
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "HiberbootEnabled", 0),
	// Visual Studio 2017-2022: PerfWatson2 (VSCEIP; telemetry) is intensive on resources.
	("HKLM", r"SOFTWARE\Wow6432Node\Microsoft\VSCommon\15.0\SQM", "OptIn", 0),
	("HKLM", r"SOFTWARE\Wow6432Node\Microsoft\VSCommon\16.0\SQM", "OptIn", 0),
	("HKLM", r"SOFTWARE\Wow6432Node\Microsoft\VSCommon\17.0\SQM", "OptIn", 0),
	("HKCU", r"Software\Microsoft\VisualStudio\Telemetry", "TurnOffSwitch", 1),
	// Disable various forms of telemetry.
	("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection", "AllowTelemetry", 0),
	("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection", "MaxTelemetryAllowed", 0),
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\AdvertisingInfo", "DisabledByGroupPolicy", 1),
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DataCollection", "AllowDeviceNameInTelemetry", 0),
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DataCollection", "AllowTelemetry", 0),
	// Disable Services Configuration - used to dynamically update telemetry service config.
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DataCollection", "DisableOneSettingsDownloads", 1),
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DataCollection", "DisableTelemetryOptInChangeNotification", 1),
	// Disable "Connected User Experiences and Telemetry" service.
	("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Diagnostics\DiagTrack", "ShowedToastAtLevel", 1),
	("HKLM", r"SYSTEM\CurrentControlSet\Services\DiagTrack", "Start", 4),
	// Disable "Customer Experience Improvement Program".
	("HKLM", r"SOFTWARE\Policies\Microsoft\AppV\CEIP", "CEIPEnable", 0),
	("HKLM", r"SOFTWARE\Microsoft\SQMClient\Windows", "CEIPEnable", 0),
	// Disable "Windows Error Reporting" service.
	("HKLM", r"SOFTWARE\Microsoft\Windows\Windows Error Reporting", "Disabled", 1),
	("HKLM", r"SOFTWARE\Microsoft\Windows\Windows Error Reporting", "AutoApproveOSDumps", 0),
	("HKLM", r"SOFTWARE\Microsoft\Windows\Windows Error Reporting", "DefaultConsent", 1),
	("HKLM", r"SOFTWARE\Microsoft\Windows\Windows Error Reporting", "DefaultOverrideBehavior", 0),
	("HKLM", r"SOFTWARE\Microsoft\Windows\Windows Error Reporting", "DontSendAdditionalData", 1),
	("HKLM", r"SOFTWARE\Microsoft\Windows\Windows Error Reporting", "LoggingDisabled", 1),
	("HKLM", r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", "AllOrNone", 0),
	("HKLM", r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", "IncludeKernelFaults", 0),
	("HKLM", r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", "IncludeMicrosoftApps", 0),
	("HKLM", r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", "IncludeShutdownErrs", 0),
	("HKLM", r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", "IncludeWindowsApps", 0),
	("HKLM", r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", "DoReport", 0),
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DeviceInstall\Settings", "DisableSendGenericDriverNotFoundToWER", 1),
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DeviceInstall\Settings", "DisableSendRequestAdditionalSoftwareToWER", 1),
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting", "Disabled", 1),
	("HKLM", r"SYSTEM\CurrentControlSet\Services\WerSvc", "Start", 4),
	// Disable telemetry for Tablet PC's handwriting recognition.
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\TabletPC", "PreventHandwritingDataSharing", 1),
	// Disable feedback reminders.
	("HKLM", r"Software\Policies\Microsoft\Windows\DataCollection", "DoNotShowFeedbackNotifications", 1),
	("HKCU", r"SOFTWARE\Microsoft\Siuf\Rules", "NumberOfSIUFInPeriod", 0),
	("HKCU", r"SOFTWARE\Microsoft\Siuf\Rules", "PeriodInNanoSeconds", 0),
	// Ask OneDrive to only generate network traffic if signed in.
	("HKLM", r"SOFTWARE\Microsoft\OneDrive", "PreventNetworkTrafficPreUserSignIn", 1),
	// Don't ask to change privacy settings after applying a major Windows update.
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\OOBE", "DisablePrivacyExperience", 1),
	("HKCU", r"Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo", "Enabled", 0),
	("HKCU", r"Software\Microsoft\Speech_OneCore\Settings\OnlineSpeechPrivacy", "HasAccepted", 0),
	("HKCU", r"Software\Microsoft\InputPersonalization", "RestrictImplicitInkCollection", 1),
	("HKCU", r"Software\Microsoft\InputPersonalization", "RestrictImplicitTextCollection", 1),
	// Don't send Microsoft inking and typing data to "improve suggestions".
	("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\TextInput", "AllowLinguisticDataCollection", 0),
	// Disable SmartScreen's Enhanced Phishing Protection (akin to Recall functionality).
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WTDS\Components", "ServiceEnabled", 0),
	// Disable the language list fingerprinting method.
	("HKCU", r"Control Panel\International\User Profile", "HttpAcceptLanguageOptOut", 1),
	// Enable multiple processes for explorer.exe for increased stability and performance.
	("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "SeparateProcess", 1),
	// Hidden file extensions are abused to hide the real file format.
	("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "HideFileExt", 0),
	// Allow usage of some .appx/.appxbundle apps without a Microsoft account.
	("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System", "MSAOptional", 1),
	// Disable all Windows Spotlight features (lock screen suggestions, tips, etc.)
	("HKCU", r"Software\Policies\Microsoft\Windows\CloudContent", "DisableWindowsSpotlightFeatures", 1),
	// Do not use diagnostic data for tailored experiences (personalized ads).
	("HKCU", r"Software\Policies\Microsoft\Windows\CloudContent", "DisableTailoredExperiencesWithDiagnosticData", 1),
	// Do not suggest third-party content in Windows Spotlight.
	("HKCU", r"Software\Policies\Microsoft\Windows\CloudContent", "DisableThirdPartySuggestions", 1),
	// Disable sync provider notifications (OneDrive ads in File Explorer).
	("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "ShowSyncProviderNotifications", 0),
	// ContentDeliveryManager controls various Windows advertisements.
	("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "ContentDeliveryAllowed", 0),
	("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SilentInstalledAppsEnabled", 0),
	("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SystemPaneSuggestionsEnabled", 0),
	("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SoftLandingEnabled", 0),
	("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "RotatingLockScreenEnabled", 0),
	("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "RotatingLockScreenOverlayEnabled", 0),
	("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "OemPreInstalledAppsEnabled", 0),
	("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "PreInstalledAppsEnabled", 0),
	("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "PreInstalledAppsEverEnabled", 0),
	("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "FeatureManagementEnabled", 0),
	("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContent-338389Enabled", 0),
	("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContent-310093Enabled", 0),
	("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContent-338388Enabled", 0),
	("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContent-338393Enabled", 0),
	("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContent-353694Enabled", 0),
	("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContent-353696Enabled", 0),
	("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContent-353698Enabled", 0),
	("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContent-88000326Enabled", 0),
];

// (hkey: "HKLM"|"HKCU", subkey, value_name, value)
const REGISTRY_STRINGS: &[(&str, &str, &str, &str)] = &[
	("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting\ExcludedApplications", "*", "*"),
	("HKLM", r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting\ExclusionList", "*", "*"),
];

// (hkey: "HKLM"|"HKCU", subkey)
const SUBKEYS_TO_REMOVE: &[(&str, &str)] = &[
	("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\StorageSense"),
	("HKLM", r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting\InclusionList"),
	("HKLM", r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting\Consent"),
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

	for (hkey_str, subkey, value_name, value) in REGISTRY_STRINGS {
		set_string(&get_hkey(hkey_str), subkey, value_name, value)?;
	}

	for (hkey_str, subkey) in SUBKEYS_TO_REMOVE {
		remove_subkey(&get_hkey(hkey_str), subkey)?;
	}

	// Double the RAM used for caching NTFS metadata.
	run_system_command("fsutil.exe", &["behavior", "set", "memoryusage", "2"])?;

	// Disable automatic repair to instead ask for a repair.
	run_system_command("bcdedit.exe", &["/set", "{default}", "recoveryenabled", "no"])?;

	// We're going to ensure this file is empty, read-only, and archived so that hiberboot cannot work.
	if Path::new(USER_NOT_PRESENT_SESSION_PATH).exists() {
		SetFileAttributes(USER_NOT_PRESENT_SESSION_PATH, FILE_ATTRIBUTE::NORMAL)?;
		fs::remove_file(USER_NOT_PRESENT_SESSION_PATH)?;
	}
	File::create(USER_NOT_PRESENT_SESSION_PATH)?;
	SetFileAttributes(
		USER_NOT_PRESENT_SESSION_PATH,
		FILE_ATTRIBUTE::ARCHIVE | FILE_ATTRIBUTE::READONLY,
	)?;

	// Random disconnection fix for specific network adapters, such as Intel's I225-V.
	run_system_command(
		"powershell.exe",
		&["-Command", "Set-NetAdapterAdvancedProperty -Name '*' -DisplayName 'Wait for Link' -RegistryValue 0"],
	)?;

	Ok(())
}
