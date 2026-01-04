use std::fs::{self, File};
use std::path::Path;

use anyhow::Result;
use winsafe::co::FILE_ATTRIBUTE;
use winsafe::{HKEY, SetFileAttributes};

use super::disable_telemetry::ENV_VARS;
use w11boost::{delete_value, run_system_command, set_dword};

// Policy values to delete; these don't exist on a fresh Windows install
const POLICY_VALUES_TO_DELETE: &[(&str, &str, &str)] = &[
        // disable_copilot.rs
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsCopilot", "TurnOffWindowsCopilot"),
        // disable_recall.rs
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI", "AllowRecallEnablement"),
        // minimize_forensics.rs
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "AITEnable"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisablePCA"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisableEngine"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "SbEnable"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisableUAR"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisableInventory"),
        ("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisableAPISampling"),
        ("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisableApplicationFootprint"),
        ("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisableInstallTracing"),
        ("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\AppCompat", "DisableWin32AppBackup"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WDI\{9c5a40da-b965-4fc3-8781-88dd50a6299d}", "ScenarioExecutionEnabled"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Messenger\Client", "CEIP"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\EdgeUI", "DisableMFUTracking"),
        ("HKCU", r"Software\Policies\Microsoft\Windows\EdgeUI", "DisableMFUTracking"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "EnableActivityFeed"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "PublishUserActivities"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "UploadUserActivities"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\FileHistory", "Disabled"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "NoSaveSettings"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "NoRecentDocsMenu"),
        ("HKLM", r"Software\Policies\Microsoft\Windows\System", "AllowClipboardHistory"),
        ("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "DisableSearchHistory"),
        ("HKCU", r"Software\Policies\Microsoft\Windows\Explorer", "NoRemoteDestinations"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting", "LoggingDisabled"),
        ("HKCU", r"SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting", "LoggingDisabled"),
        ("HKLM", r"Software\Policies\Microsoft\Windows\HandwritingErrorReports", "PreventHandwritingErrorReports"),
        ("HKLM", r"Software\Policies\Microsoft\Windows\DataCollection", "LimitDiagnosticLogCollection"),
        ("HKLM", r"Software\Policies\Microsoft\Windows\DataCollection", "LimitDumpCollection"),
        // minimize_online_data_collection.rs
        ("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer\AllowOnlineTips", "AllowOnlineTips"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\InputPersonalization", "AllowInputPersonalization"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "HideRecommendedPersonalizedSites"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Explorer", "HideRecommendedSection"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Device Metadata", "PreventDeviceMetadataFromNetwork"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\SearchCompanion", "DisableContentFileUpdates"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "AllowCrossDeviceClipboard"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\StorageHealth", "AllowDiskHealthModelUpdates"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Appx", "DisableBackgroundAutoUpdates"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\CloudContent", "DisableCloudOptimizedContent"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\CloudContent", "DisableConsumerAccountStateContent"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\CloudContent", "DisableSoftLanding"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\CloudContent", "DisableWindowsConsumerFeatures"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Messaging", "AllowMessageSync"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Search", "AllowCortana"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Search", "AllowCloudSearch"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Search", "DisableWebSearch"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Search", "EnableDynamicContentInWSB"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Search", "ConnectedSearchUseWeb"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Speech", "AllowSpeechModelUpdate"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Dsh", "AllowNewsAndInterests"),
        ("HKLM", r"Software\Policies\Microsoft\Windows\Personalization", "NoLockScreen"),
        // non_intrusive_tweaks.rs
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Appx", "AllowAutomaticAppArchiving"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\SQMClient\Windows", "CEIPEnable"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\AdvertisingInfo", "DisabledByGroupPolicy"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DataCollection", "AllowDeviceNameInTelemetry"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DataCollection", "AllowTelemetry"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DataCollection", "DisableOneSettingsDownloads"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DataCollection", "DisableTelemetryOptInChangeNotification"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\AppV\CEIP", "CEIPEnable"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", "AllOrNone"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", "IncludeKernelFaults"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", "IncludeMicrosoftApps"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", "IncludeShutdownErrs"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", "IncludeWindowsApps"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting", "DoReport"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DeviceInstall\Settings", "DisableSendGenericDriverNotFoundToWER"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\DeviceInstall\Settings", "DisableSendRequestAdditionalSoftwareToWER"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting", "Disabled"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\TabletPC", "PreventHandwritingDataSharing"),
        ("HKLM", r"Software\Policies\Microsoft\Windows\DataCollection", "DoNotShowFeedbackNotifications"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\OOBE", "DisablePrivacyExperience"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\WTDS\Components", "ServiceEnabled"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\System", "HiberbootEnabled"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Appx", "AllowStorageSenseGlobal"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\StorageSense", "AllowStorageSenseGlobal"),
        ("HKCU", r"Software\Policies\Microsoft\Windows\CloudContent", "DisableWindowsSpotlightFeatures"),
        ("HKCU", r"Software\Policies\Microsoft\Windows\CloudContent", "DisableTailoredExperiencesWithDiagnosticData"),
        ("HKCU", r"Software\Policies\Microsoft\Windows\CloudContent", "DisableThirdPartySuggestions"),
        // disable_telemetry.rs
        ("HKLM", r"Software\Policies\Mozilla\Firefox", "DisableTelemetry"),
        ("HKCU", r"Software\Policies\Mozilla\Firefox", "DisableTelemetry"),
        ("HKCU", r"Software\Policies\Microsoft\office\16.0\common\privacy", "SendTelemetry"),
];

// Policy subkeys to delete entirely
const POLICY_SUBKEYS_TO_DELETE: &[(&str, &str)] = &[
        // disable_sleep.rs - Power settings GUIDs
        ("HKLM", r"Software\Policies\Microsoft\Power\PowerSettings\94ac6d29-73ce-41a6-809f-6363ba21b47e"),
        ("HKLM", r"Software\Policies\Microsoft\Power\PowerSettings\29F6C1DB-86DA-48C5-9FDB-F2B67B1F44DA"),
        ("HKLM", r"Software\Policies\Microsoft\Power\PowerSettings\9D7815A6-7EE4-497E-8888-515A05F02364"),
        ("HKLM", r"Software\Policies\Microsoft\Power\PowerSettings\7bc4a2f9-d8fc-4469-b07b-33eb785aaca0"),
        // non_intrusive_tweaks.rs - Error reporting subkeys
        ("HKLM", r"SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting\ExcludedApplications"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting\ExclusionList"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting\InclusionList"),
        ("HKLM", r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting\Consent"),
];

// Non-policy values to delete (don't exist on fresh Windows)
const SETTINGS_TO_DELETE: &[(&str, &str, &str)] = &[
        // disable_sleep.rs
        ("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\FlyoutMenuSettings", "ShowHibernateOption"),
        ("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\FlyoutMenuSettings", "ShowSleepOption"),
        // minimize_forensics.rs
        ("HKLM", r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Perflib", "Disable Performance Counters"),
        ("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoRecentDocsHistory"),
        ("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoStartMenuMFUprogramsList"),
        ("HKCU", r"SYSTEM\CurrentControlSet\Control\Session Manager\Configuration Manager", "EnablePeriodicBackup"),
        ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "LinkResolveIgnoreLinkInfo"),
        ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoResolveSearch"),
        ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoResolveTrack"),
        ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\comdlg32", "NoFileMru"),
        ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\comdlg32", "NoBackButton"),
        ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "ClearRecentDocsOnExit"),
        ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "ClearRecentProgForNewUserInStartMenu"),
        ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer", "NoRecentDocsNetHood"),
        ("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Management", "EnableSuperfetch"),
        ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "Start_TrackProgs"),
        ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\UserAssist\{CEBFF5CD-ACE2-4F4F-9178-9926F41749EA}\Settings", "NoLog"),
        ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\UserAssist\{F4E57C4B-2036-45F0-A9AB-443BCFE33D9F}\Settings", "NoLog"),
        ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "DisableThumbnailCache"),
        ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "DisableThumbsDBOnNetworkFolders"),
        ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "DontUsePowerShellOnWinX"),
        ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\SearchSettings", "IsDeviceSearchHistoryEnabled"),
        // non_intrusive_tweaks.rs
        ("HKCU", r"SOFTWARE\Microsoft\Siuf\Rules", "PeriodInNanoSeconds"),
        ("HKLM", r"SYSTEM\CurrentControlSet\Policies", "NtfsForceNonPagedPoolAllocation"),
        ("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System", "verbosestatus"),
        ("HKLM", r"SOFTWARE\Microsoft\PolicyManager\current\device\System", "AllowExperimentation"),
        ("HKLM", r"SYSTEM\CurrentControlSet\Control\Power\PowerThrottling", "PowerThrottlingOff"),
        ("HKLM", r"SOFTWARE\Wow6432Node\Microsoft\VSCommon\15.0\SQM", "OptIn"),
        ("HKLM", r"SOFTWARE\Wow6432Node\Microsoft\VSCommon\16.0\SQM", "OptIn"),
        ("HKLM", r"SOFTWARE\Wow6432Node\Microsoft\VSCommon\17.0\SQM", "OptIn"),
        ("HKCU", r"Software\Microsoft\VisualStudio\Telemetry", "TurnOffSwitch"),
        ("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Diagnostics\DiagTrack", "ShowedToastAtLevel"),
        ("HKLM", r"SOFTWARE\Microsoft\Windows\Windows Error Reporting", "Disabled"),
        ("HKLM", r"SOFTWARE\Microsoft\Windows\Windows Error Reporting", "AutoApproveOSDumps"),
        ("HKLM", r"SOFTWARE\Microsoft\Windows\Windows Error Reporting", "DefaultConsent"),
        ("HKLM", r"SOFTWARE\Microsoft\Windows\Windows Error Reporting", "DontSendAdditionalData"),
        ("HKLM", r"SOFTWARE\Microsoft\Windows\Windows Error Reporting", "LoggingDisabled"),
        ("HKCU", r"SOFTWARE\Microsoft\Siuf\Rules", "NumberOfSIUFInPeriod"),
        ("HKLM", r"SOFTWARE\Microsoft\OneDrive", "PreventNetworkTrafficPreUserSignIn"),
        ("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\TextInput", "AllowLinguisticDataCollection"),
        ("HKCU", r"Control Panel\International\User Profile", "HttpAcceptLanguageOptOut"),
        ("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System", "MSAOptional"),
        ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "ShowSyncProviderNotifications"),
        ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "PreInstalledAppsEverEnabled"),
        ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContent-338388Enabled"),
        ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContent-338393Enabled"),
        ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContent-353696Enabled"),
        ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContent-88000326Enabled"),
];

// Non-policy settings to restore to Windows defaults (only values that exist on stock Windows)
const SETTINGS_TO_RESTORE: &[(&str, &str, &str, u32)] = &[
        // disable_sleep.rs
        ("HKLM", r"SYSTEM\CurrentControlSet\Control\Power", "HibernateEnabledDefault", 1),
        // minimize_forensics.rs
        ("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\PrefetchParameters", "EnablePrefetcher", 3),
        // non_intrusive_tweaks.rs
        ("HKLM", r"SOFTWARE\Microsoft\FTH", "Enabled", 1),
        ("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "DisablePagingExecutive", 0),
        ("HKLM", r"SYSTEM\CurrentControlSet\Control\Session Manager\Power", "HiberbootEnabled", 1),
        ("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection", "AllowTelemetry", 3),
        ("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection", "MaxTelemetryAllowed", 3),
        ("HKLM", r"SOFTWARE\Microsoft\SQMClient\Windows", "CEIPEnable", 0),
        ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo", "Enabled", 0),
        ("HKCU", r"Software\Microsoft\InputPersonalization", "RestrictImplicitInkCollection", 0),
        ("HKCU", r"Software\Microsoft\InputPersonalization", "RestrictImplicitTextCollection", 0),
        ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "SeparateProcess", 0),
        ("HKCU", r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "HideFileExt", 1),
        // ContentDeliveryManager - values that exist on stock Windows
        ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "ContentDeliveryAllowed", 1),
        ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SilentInstalledAppsEnabled", 1),
        ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SystemPaneSuggestionsEnabled", 1),
        ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SoftLandingEnabled", 1),
        ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "RotatingLockScreenEnabled", 1),
        ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "RotatingLockScreenOverlayEnabled", 1),
        ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "OemPreInstalledAppsEnabled", 1),
        ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "PreInstalledAppsEnabled", 1),
        ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "FeatureManagementEnabled", 0),
        ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContent-338389Enabled", 0),
        ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContent-310093Enabled", 0),
        ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContent-353694Enabled", 0),
        ("HKCU", r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", "SubscribedContent-353698Enabled", 0),
];

// Services to restore to default Start values
// 1=System, 2=Auto, 3=Manual, 4=Disabled
const SERVICES_TO_RESTORE: &[(&str, u32)] = &[
        ("SysMain", 2),       // Superfetch - Auto
        ("PcaSvc", 2),        // Program Compatibility Assistant - Auto
        ("bam", 1),           // Background Activity Moderator - System
        ("dam", 1),           // Desktop Activity Moderator - System
        ("dmwappushservice", 3), // WAP Push - Manual
        ("diagsvc", 3),       // Diagnostic Execution Service - Manual
        ("DiagTrack", 2),     // Connected User Experiences and Telemetry - Auto
        ("WerSvc", 3),        // Windows Error Reporting - Manual
];

fn get_hkey(hkey_str: &str) -> HKEY
{
        if hkey_str == "HKLM" { HKEY::LOCAL_MACHINE } else { HKEY::CURRENT_USER }
}

pub fn run() -> Result<()>
{
        let hklm = HKEY::LOCAL_MACHINE;
        let hkcu = HKEY::CURRENT_USER;

        // 1. Delete policy values (they don't exist on fresh Windows)
        for (hkey_str, subkey, value_name) in POLICY_VALUES_TO_DELETE {
                let _ = delete_value(&get_hkey(hkey_str), subkey, value_name);
        }

        // 2. Delete policy subkeys entirely
        for (hkey_str, subkey) in POLICY_SUBKEYS_TO_DELETE {
                let _ = get_hkey(hkey_str).RegDeleteTree(Some(subkey));
        }

        // 3. Delete non-policy settings that don't exist by default
        for (hkey_str, subkey, value_name) in SETTINGS_TO_DELETE {
                let _ = delete_value(&get_hkey(hkey_str), subkey, value_name);
        }

        // 4. Restore non-policy settings to Windows defaults
        for (hkey_str, subkey, value_name, default_value) in SETTINGS_TO_RESTORE {
                let _ = set_dword(&get_hkey(hkey_str), subkey, value_name, *default_value);
        }

        // 5. Restore services to default Start values
        for (service_name, default_start) in SERVICES_TO_RESTORE {
                let subkey = format!(r"SYSTEM\CurrentControlSet\Services\{}", service_name);
                let _ = set_dword(&hklm, &subkey, "Start", *default_start);
        }

        // 6. Delete environment variables from disable_telemetry.rs
        for (var_name, _) in ENV_VARS {
                let _ = delete_value(&hkcu, "Environment", var_name);
        }

        // 7. Reset NTFS memory usage to default
        run_system_command("fsutil.exe", &["behavior", "set", "memoryusage", "1"])?;

        // 8. Re-enable last access timestamps
        run_system_command("fsutil.exe", &["behavior", "set", "disablelastaccess", "2"])?;

        // 9. Re-enable automatic recovery
        run_system_command("bcdedit.exe", &["/set", "{default}", "recoveryenabled", "yes"])?;

        // 10. Reset UserNotPresentSession.etl so Fast Startup can work again
        let file_path = r"C:\Windows\System32\SleepStudy\UserNotPresentSession.etl";

        if Path::new(file_path).exists() {
                SetFileAttributes(file_path, FILE_ATTRIBUTE::NORMAL)?;
                fs::remove_file(file_path)?;
        }

        File::create(file_path)?;

        // 11. Reset "Wait for Link" adapter setting (may silently fail on unsupported adapters)
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
