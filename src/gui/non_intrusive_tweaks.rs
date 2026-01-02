use crate::common::{remove_subkey, run_system_command, set_dword, set_string};
use anyhow::Result;
use std::{
        fs::{self, File},
        path::Path,
};
use winsafe::{HKEY, SetFileAttributes, co::FILE_ATTRIBUTE};

pub fn run() -> Result<()>
{
        let hklm = HKEY::LOCAL_MACHINE;
        let hkcu = HKEY::CURRENT_USER;

        // If allowed (1): unused apps would be uninstalled with their user data left intact, then reinstalled if launched afterwards at any point in time.
        set_dword(
                &hklm,
                r"SOFTWARE\Policies\Microsoft\Windows\Appx",
                "AllowAutomaticAppArchiving",
                0,
        )?;

        // Make all users opted out of the Windows Customer Experience Improvement Program.
        set_dword(&hklm, r"SOFTWARE\Policies\Microsoft\SQMClient\Windows", "CEIPEnable", 0)?;

        // Shows what's slowing down bootups and shutdowns.
        set_dword(
                &hklm,
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System",
                "verbosestatus",
                1,
        )?;

        // Ask to not allow execution of experiments by Microsoft.
        set_dword(
                &hklm,
                r"SOFTWARE\Microsoft\PolicyManager\current\device\System",
                "AllowExperimentation",
                0,
        )?;

        // Power Throttling causes severe performance reduction for VMWare Workstation 17.
        set_dword(
                &hklm,
                r"SYSTEM\CurrentControlSet\Control\Power\PowerThrottling",
                "PowerThrottlingOff",
                1,
        )?;

        // https://docs.microsoft.com/en-us/windows/desktop/win7appqual/fault-tolerant-heap
        // FTH being enabled causes issues with specific apps such as Assetto Corsa.
        set_dword(&hklm, r"SOFTWARE\Microsoft\FTH", "Enabled", 0)?;

        // Automated file cleanup without user interaction is a bad idea, even if ran only on low-disk space events.
        set_dword(
                &hklm,
                r"SOFTWARE\Policies\Microsoft\Windows\Appx",
                "AllowStorageSenseGlobal",
                0,
        )?;

        set_dword(
                &hklm,
                r"SOFTWARE\Policies\Microsoft\Windows\StorageSense",
                "AllowStorageSenseGlobal",
                0,
        )?;

        remove_subkey(&hklm, r"SOFTWARE\Microsoft\Windows\CurrentVersion\StorageSense")?;

        // Allocate more RAM to NTFS' paged pool.
        set_dword(
                &hklm,
                r"SYSTEM\CurrentControlSet\Policies",
                "NtfsForceNonPagedPoolAllocation",
                1,
        )?;

        // Double the RAM used for caching NTFS metadata.
        run_system_command("fsutil.exe", &["behavior", "set", "memoryusage", "2"])?;

        // Disable automatic repair to instead ask for a repair.
        // Does not disable Windows' Recovery environment thankfully.
        run_system_command("bcdedit.exe", &["/set", "{default}", "recoveryenabled", "no"])?;

        // Do not page drivers and other system code to a disk, keep it in memory.
        set_dword(
                &hklm,
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
                "DisablePagingExecutive",
                1,
        )?;

        // Disables "Fast startup", a form of hibernation.
        set_dword(
                &hklm,
                r"SYSTEM\CurrentControlSet\Control\Session Manager\Power",
                "HiberbootEnabled",
                0,
        )?;

        // Do not require use of fast startup.
        set_dword(
                &hklm,
                r"SOFTWARE\Policies\Microsoft\Windows\System",
                "HiberbootEnabled",
                0,
        )?;

        // We're going to ensure this file is empty, read-only, and archived so that hiberboot cannot work.
        let file_path = r"C:\Windows\System32\SleepStudy\UserNotPresentSession.etl";

        if Path::new(file_path).exists() {
                // Removing UserNotPresentSession.etl will fail if it is readonly.
                SetFileAttributes(file_path, FILE_ATTRIBUTE::NORMAL)?;
                fs::remove_file(file_path)?;
        }

        File::create(file_path)?;

        SetFileAttributes(file_path, FILE_ATTRIBUTE::ARCHIVE | FILE_ATTRIBUTE::READONLY)?;

        // Visual Studio 2017 up to 2022: PerfWatson2 (VSCEIP; telemetry) is intensive on resources, disable it.
        set_dword(&hklm, r"SOFTWARE\Wow6432Node\Microsoft\VSCommon\15.0\SQM", "OptIn", 0)?;
        set_dword(&hklm, r"SOFTWARE\Wow6432Node\Microsoft\VSCommon\16.0\SQM", "OptIn", 0)?;
        set_dword(&hklm, r"SOFTWARE\Wow6432Node\Microsoft\VSCommon\17.0\SQM", "OptIn", 0)?;
        // Visual Studio: disable other telemetry.
        set_dword(&hkcu, r"Software\Microsoft\VisualStudio\Telemetry", "TurnOffSwitch", 1)?;

        // Disable various forms of telemetry.
        set_dword(
                &hklm,
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection",
                "AllowTelemetry",
                0,
        )?;
        set_dword(
                &hklm,
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection",
                "MaxTelemetryAllowed",
                0,
        )?;
        set_dword(
                &hklm,
                r"SOFTWARE\Policies\Microsoft\Windows\AdvertisingInfo",
                "DisabledByGroupPolicy",
                1,
        )?;
        set_dword(
                &hklm,
                r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
                "AllowDeviceNameInTelemetry",
                0,
        )?;
        set_dword(
                &hklm,
                r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
                "AllowTelemetry",
                0,
        )?;

        // Disable Services Configuration.
        // Used by Windows components and apps, such as the telemetry service, to dynamically update their configuration.
        set_dword(
                &hklm,
                r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
                "DisableOneSettingsDownloads",
                1,
        )?;

        set_dword(
                &hklm,
                r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
                "DisableTelemetryOptInChangeNotification",
                1,
        )?;

        // Disable "Connected User Experiences and Telemetry" service
        set_dword(
                &hklm,
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Diagnostics\DiagTrack",
                "ShowedToastAtLevel",
                1,
        )?;
        set_dword(&hklm, r"SYSTEM\CurrentControlSet\Services\DiagTrack", "Start", 4)?;

        // Disable "Customer Experience Improvement Program"; also implies turning off the Inventory Collector.
        set_dword(&hklm, r"SOFTWARE\Policies\Microsoft\AppV\CEIP", "CEIPEnable", 0)?;
        set_dword(&hklm, r"SOFTWARE\Microsoft\SQMClient\Windows", "CEIPEnable", 0)?;

        //--START-- Disable "Windows Error Reporting" service
        let wer = r"SOFTWARE\Microsoft\Windows\Windows Error Reporting";
        set_dword(&hklm, wer, "Disabled", 1)?;
        set_dword(&hklm, wer, "AutoApproveOSDumps", 0)?;
        set_dword(&hklm, wer, "DefaultConsent", 1)?;
        set_dword(&hklm, wer, "DefaultOverrideBehavior", 0)?;
        set_dword(&hklm, wer, "DontSendAdditionalData", 1)?;
        set_dword(&hklm, wer, "LoggingDisabled", 1)?;

        let error_reporting = r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting";
        set_dword(&hklm, error_reporting, "AllOrNone", 0)?;
        set_dword(&hklm, error_reporting, "IncludeKernelFaults", 0)?;
        set_dword(&hklm, error_reporting, "IncludeMicrosoftApps", 0)?;
        set_dword(&hklm, error_reporting, "IncludeShutdownErrs", 0)?;
        set_dword(&hklm, error_reporting, "IncludeWindowsApps", 0)?;
        set_dword(&hklm, error_reporting, "DoReport", 0)?;

        set_dword(
                &hklm,
                r"SOFTWARE\Policies\Microsoft\Windows\DeviceInstall\Settings",
                "DisableSendGenericDriverNotFoundToWER",
                1,
        )?;
        set_dword(
                &hklm,
                r"SOFTWARE\Policies\Microsoft\Windows\DeviceInstall\Settings",
                "DisableSendRequestAdditionalSoftwareToWER",
                1,
        )?;

        set_dword(
                &hklm,
                r"SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting",
                "Disabled",
                1,
        )?;

        set_string(
                &hklm,
                r"SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting\ExcludedApplications",
                "*",
                "*",
        )?;
        set_string(
                &hklm,
                r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting\ExclusionList",
                "*",
                "*",
        )?;

        set_dword(&hklm, r"SYSTEM\CurrentControlSet\Services\WerSvc", "Start", 4)?;

        remove_subkey(
                &hklm,
                r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting\InclusionList",
        )?;
        remove_subkey(&hklm, r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting\Consent")?;
        //--END-- Disable "Windows Error Reporting" service

        // Disable telemetry for Tablet PC's handwriting recognition
        set_dword(
                &hklm,
                r"SOFTWARE\Policies\Microsoft\Windows\TabletPC",
                "PreventHandwritingDataSharing",
                1,
        )?;

        // Disable feedback reminders.
        set_dword(
                &hklm,
                r"Software\Policies\Microsoft\Windows\DataCollection",
                "DoNotShowFeedbackNotifications",
                1,
        )?;
        set_dword(&hkcu, r"SOFTWARE\Microsoft\Siuf\Rules", "NumberOfSIUFInPeriod", 0)?;
        set_dword(&hkcu, r"SOFTWARE\Microsoft\Siuf\Rules", "PeriodInNanoSeconds", 0)?;

        // Ask OneDrive to only generate network traffic if signed in to OneDrive
        set_dword(
                &hklm,
                r"SOFTWARE\Microsoft\OneDrive",
                "PreventNetworkTrafficPreUserSignIn",
                1,
        )?;

        // Don't ask to change the current privacy settings after applying a major
        // Windows update
        set_dword(
                &hklm,
                r"SOFTWARE\Policies\Microsoft\Windows\OOBE",
                "DisablePrivacyExperience",
                1,
        )?;

        set_dword(
                &hkcu,
                r"Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo",
                "Enabled",
                0,
        )?;
        set_dword(
                &hkcu,
                r"Software\Microsoft\Speech_OneCore\Settings\OnlineSpeechPrivacy",
                "HasAccepted",
                0,
        )?;
        set_dword(
                &hkcu,
                r"Software\Microsoft\InputPersonalization",
                "RestrictImplicitInkCollection",
                1,
        )?;
        set_dword(
                &hkcu,
                r"Software\Microsoft\InputPersonalization",
                "RestrictImplicitTextCollection",
                1,
        )?;
        // Don't send Microsoft inking and typing data to "improve suggestions".
        set_dword(
                &hklm,
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\TextInput",
                "AllowLinguisticDataCollection",
                0,
        )?;

        // Disable SmartScreen's Enhanced Phishing Protection; it's akin to Microsoft's Recall functionality.
        set_dword(
                &hklm,
                r"SOFTWARE\Policies\Microsoft\Windows\WTDS\Components",
                "ServiceEnabled",
                0,
        )?;

        // Disable the language list fingerprinting method; useful for bypassing geolocation restrictions.
        set_dword(
                &hkcu,
                r"Control Panel\International\User Profile",
                "HttpAcceptLanguageOptOut",
                1,
        )?;

        // Enable multiple processes for explorer.exe for increased stability and performance.
        set_dword(
                &hkcu,
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                "SeparateProcess",
                1,
        )?;

        // Hidden file extensions are abused to hide the real file format, example:
        // an executable (.exe, .scr, .com) pretending to be a PDF.
        set_dword(
                &hkcu,
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                "HideFileExt",
                0,
        )?;

        // Allow usage of some .appx/.appxbundle apps without a Microsoft account.
        set_dword(
                &hklm,
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System",
                "MSAOptional",
                1,
        )?;

        // Random disconnection fix for specific network adapters, such as Intel's I225-V.
        run_system_command(
                "powershell.exe",
                &[
                        "-Command",
                        "Set-NetAdapterAdvancedProperty -Name '*' -DisplayName 'Wait for Link' -RegistryValue 0",
                ],
        )?;

        // ============================================================================
        // Disable Windows advertisements and suggestions
        // ============================================================================

        // Disable all Windows Spotlight features (lock screen suggestions, tips, etc.)
        set_dword(
                &hkcu,
                r"Software\Policies\Microsoft\Windows\CloudContent",
                "DisableWindowsSpotlightFeatures",
                1,
        )?;

        // Do not use diagnostic data for tailored experiences (personalized ads).
        set_dword(
                &hkcu,
                r"Software\Policies\Microsoft\Windows\CloudContent",
                "DisableTailoredExperiencesWithDiagnosticData",
                1,
        )?;

        // Do not suggest third-party content in Windows Spotlight.
        set_dword(
                &hkcu,
                r"Software\Policies\Microsoft\Windows\CloudContent",
                "DisableThirdPartySuggestions",
                1,
        )?;

        // Disable sync provider notifications (OneDrive ads in File Explorer).
        set_dword(
                &hkcu,
                r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
                "ShowSyncProviderNotifications",
                0,
        )?;

        // ContentDeliveryManager controls various Windows advertisements.
        let cdm = r"SOFTWARE\Microsoft\Windows\CurrentVersion\ContentDeliveryManager";

        // Master switch for content delivery.
        set_dword(&hkcu, cdm, "ContentDeliveryAllowed", 0)?;

        // Disable silent app installations.
        set_dword(&hkcu, cdm, "SilentInstalledAppsEnabled", 0)?;

        // Disable suggestions in Settings app.
        set_dword(&hkcu, cdm, "SystemPaneSuggestionsEnabled", 0)?;

        // Disable "soft landing" promotional content.
        set_dword(&hkcu, cdm, "SoftLandingEnabled", 0)?;

        // Disable rotating lock screen images (Spotlight).
        set_dword(&hkcu, cdm, "RotatingLockScreenEnabled", 0)?;
        set_dword(&hkcu, cdm, "RotatingLockScreenOverlayEnabled", 0)?;

        // Disable OEM and pre-installed app suggestions.
        set_dword(&hkcu, cdm, "OemPreInstalledAppsEnabled", 0)?;
        set_dword(&hkcu, cdm, "PreInstalledAppsEnabled", 0)?;
        set_dword(&hkcu, cdm, "PreInstalledAppsEverEnabled", 0)?;

        // Disable feature management (used for A/B testing and feature rollouts).
        set_dword(&hkcu, cdm, "FeatureManagementEnabled", 0)?;

        // Disable "Get tips, tricks, and suggestions" notifications.
        set_dword(&hkcu, cdm, "SubscribedContent-338389Enabled", 0)?;

        // Disable "Show me the Windows welcome experience" after updates.
        set_dword(&hkcu, cdm, "SubscribedContent-310093Enabled", 0)?;

        // Disable suggestions in Start menu.
        set_dword(&hkcu, cdm, "SubscribedContent-338388Enabled", 0)?;

        // Disable suggestions in timeline.
        set_dword(&hkcu, cdm, "SubscribedContent-338393Enabled", 0)?;

        // Disable suggested content in Settings app.
        set_dword(&hkcu, cdm, "SubscribedContent-353694Enabled", 0)?;
        set_dword(&hkcu, cdm, "SubscribedContent-353696Enabled", 0)?;
        set_dword(&hkcu, cdm, "SubscribedContent-353698Enabled", 0)?;

        // Disable "Suggest ways to get the most out of Windows" notification.
        set_dword(&hkcu, cdm, "SubscribedContent-88000326Enabled", 0)?;

        Ok(())
}
