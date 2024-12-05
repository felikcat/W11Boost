use crate::common::*;
use std::{
    error::Error,
    fs::{self, File},
    process::Command,
};
use winsafe::{HKEY, SetFileAttributes, co::FILE_ATTRIBUTE, prelude::advapi_Hkey};

pub fn run() -> Result<(), Box<dyn Error>> {
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
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\SQMClient\Windows",
        "CEIPEnable",
        0,
    )?;

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

    // Power Throttling causes severe performance reduction for VMWare.
    // Workstation 17
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

    remove_subkey(
        &hklm,
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\StorageSense",
    )?;

    // Allocate more RAM to NTFS' paged pool.
    set_dword(
        &hklm,
        r"SYSTEM\CurrentControlSet\Policies",
        "NtfsForceNonPagedPoolAllocation",
        1,
    )?;

    Command::new("fsutil.exe")
        .args(["behavior", "set", "memoryusage", "2"])
        .output()?;

    // Disable automatic repair to instead ask for a repair.
    // Does not disable Windows' Recovery environment thankfully.
    Command::new("bcdedit.exe")
        .args(["/set", "{default}", "recoveryenabled", "no"])
        .output()?;

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

    let file_path = r"C:\Windows\System32\SleepStudy\UserNotPresentSession.etl";

    // Removing UserNotPresentSession.etl will fail if it is readonly.
    SetFileAttributes(
        file_path,
        FILE_ATTRIBUTE::NORMAL,
    )
    .expect("SetFileAttributes 1 failed");

    fs::remove_file(file_path).expect("Failed to remove the UserNotPresentSession.etl file");

    File::create(file_path).expect("UserNotPresentSession.etl file creation failed");

    SetFileAttributes(
        file_path,
        FILE_ATTRIBUTE::ARCHIVE | FILE_ATTRIBUTE::READONLY,
    )
    .expect("SetFileAttributes 2 failed");

    // Visual Studio 2017 up to 2022: PerfWatson2 (VSCEIP; telemetry) is intensive on resources, disable it.
    set_dword(
        &hklm,
        r"SOFTWARE\Wow6432Node\Microsoft\VSCommon\15.0\SQM",
        "OptIn",
        0,
    )?;
    set_dword(
        &hklm,
        r"SOFTWARE\Wow6432Node\Microsoft\VSCommon\16.0\SQM",
        "OptIn",
        0,
    )?;
    set_dword(
        &hklm,
        r"SOFTWARE\Wow6432Node\Microsoft\VSCommon\17.0\SQM",
        "OptIn",
        0,
    )?;
    // Visual Studio: disable other telemetry.
    set_dword(
        &hkcu,
        r"Software\Microsoft\VisualStudio\Telemetry",
        "TurnOffSwitch",
        1,
    )?;

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
    set_dword(
        &hklm,
        r"SYSTEM\CurrentControlSet\Services\DiagTrack",
        "Start",
        4,
    )?;

    // Disable "Diagnostic Policy Service"
    // Logs tons of information to be sent off and analyzed by Microsoft, and in some cases caused noticeable performance slowdown.
    set_dword(&hklm, r"SYSTEM\CurrentControlSet\Services\DPS", "Start", 4)?;

    // Disable "Customer Experience Improvement Program"; also implies turning off the Inventory Collector.
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\AppV\CEIP",
        "CEIPEnable",
        0,
    )?;
    set_dword(
        &hklm,
        r"SOFTWARE\Microsoft\SQMClient\Windows",
        "CEIPEnable",
        0,
    )?;

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

    set_dword(
        &hklm,
        r"SYSTEM\CurrentControlSet\Services\WerSvc",
        "Start",
        4,
    )?;

    remove_subkey(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting\InclusionList",
    )?;
    remove_subkey(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting\Consent",
    )?;
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
    set_dword(
        &hkcu,
        r"SOFTWARE\Microsoft\Siuf\Rules",
        "NumberOfSIUFInPeriod",
        0,
    )?;
    set_dword(
        &hkcu,
        r"SOFTWARE\Microsoft\Siuf\Rules",
        "PeriodInNanoSeconds",
        0,
    )?;

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

    // Random disconnection fix for specific network adapters, such as Intel's I225-V.
    Command::new("powershell.exe")
    .args([
        "-Command",
        "Set-NetAdapterAdvancedProperty -Name '*' -DisplayName 'Wait for Link' -RegistryValue 0"
    ]).output().expect("Setting network adapter advanced property failed");

    Ok(())
}
