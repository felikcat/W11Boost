use crate::common::*;
use std::{
    error::Error,
    fs::{self, File},
    process::Command,
};
use windows::{core::w, Win32::System::{GroupPolicy::IGroupPolicyObject, Registry::{HKEY, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE}}};
use winsafe::{co::FILE_ATTRIBUTE, prelude::advapi_Hkey, SetFileAttributes, HKEY as HKEY_SAFE};

pub fn run() -> Result<(), Box<dyn Error>> {
    let (hklm, gpo_hklm): (HKEY, IGroupPolicyObject) = init_registry_gpo(HKEY_LOCAL_MACHINE)?;
    let (hkcu, gpo_hkcu): (HKEY, IGroupPolicyObject) = init_registry_gpo(HKEY_CURRENT_USER)?;
    let hklm_safe = HKEY_SAFE::LOCAL_MACHINE;

    // If allowed (1): unused apps would be uninstalled with their user data left intact, then reinstalled if launched afterwards at any point in time.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\Appx"),
        w!("AllowAutomaticAppArchiving"),
        0,
    )?;

    // Make all users opted out of the Windows Customer Experience Improvement Program.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\SQMClient\Windows"),
        w!("CEIPEnable"),
        0,
    )?;

    // Shows what's slowing down bootups and shutdowns.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System"),
        w!("verbosestatus"),
        1,
    )?;

    // Ask to not allow execution of experiments by Microsoft.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Microsoft\PolicyManager\current\device\System"),
        w!("AllowExperimentation"),
        0,
    )?;

    // Power Throttling causes severe performance reduction for VMWare.
    // Workstation 17
    set_dword_gpo(
        hklm,
        w!(r"SYSTEM\CurrentControlSet\Control\Power\PowerThrottling"),
        w!("PowerThrottlingOff"),
        1,
    )?;

    // https://docs.microsoft.com/en-us/windows/desktop/win7appqual/fault-tolerant-heap
    // FTH being enabled causes issues with specific apps such as Assetto Corsa.
    set_dword_gpo(hklm, w!(r"SOFTWARE\Microsoft\FTH"), w!("Enabled"), 0)?;

    // Automated file cleanup without user interaction is a bad idea, even if ran only on low-disk space events.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\Appx"),
        w!("AllowStorageSenseGlobal"),
        0,
    )?;

    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\StorageSense"),
        w!("AllowStorageSenseGlobal"),
        0,
    )?;

    remove_subkey(
        &hklm_safe,
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\StorageSense",
    )?;

    // Allocate more RAM to NTFS' paged pool.
    set_dword_gpo(
        hklm,
        w!(r"SYSTEM\CurrentControlSet\Policies"),
        w!("NtfsForceNonPagedPoolAllocation"),
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
    set_dword_gpo(
        hklm,
        w!(r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management"),
        w!("DisablePagingExecutive"),
        1,
    )?;

    // Disables "Fast startup", a form of hibernation.
    set_dword_gpo(
        hklm,
        w!(r"SYSTEM\CurrentControlSet\Control\Session Manager\Power"),
        w!("HiberbootEnabled"),
        0,
    )?;

    // Do not require use of fast startup.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\System"),
        w!("HiberbootEnabled"),
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
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Wow6432Node\Microsoft\VSCommon\15.0\SQM"),
        w!("OptIn"),
        0,
    )?;
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Wow6432Node\Microsoft\VSCommon\16.0\SQM"),
        w!("OptIn"),
        0,
    )?;
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Wow6432Node\Microsoft\VSCommon\17.0\SQM"),
        w!("OptIn"),
        0,
    )?;
    // Visual Studio: disable other telemetry.
    set_dword_gpo(
        hkcu,
        w!(r"Software\Microsoft\VisualStudio\Telemetry"),
        w!("TurnOffSwitch"),
        1,
    )?;

    // Disable various forms of telemetry.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection"),
        w!("AllowTelemetry"),
        0,
    )?;
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\DataCollection"),
        w!("MaxTelemetryAllowed"),
        0,
    )?;
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\AdvertisingInfo"),
        w!("DisabledByGroupPolicy"),
        1,
    )?;
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\DataCollection"),
        w!("AllowDeviceNameInTelemetry"),
        0,
    )?;
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\DataCollection"),
        w!("AllowTelemetry"),
        0,
    )?;

    // Disable Services Configuration.
    // Used by Windows components and apps, such as the telemetry service, to dynamically update their configuration.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\DataCollection"),
        w!("DisableOneSettingsDownloads"),
        1,
    )?;

    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\DataCollection"),
        w!("DisableTelemetryOptInChangeNotification"),
        1,
    )?;

    // Disable "Connected User Experiences and Telemetry" service
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Diagnostics\DiagTrack"),
        w!("ShowedToastAtLevel"),
        1,
    )?;
    set_dword_gpo(
        hklm,
        w!(r"SYSTEM\CurrentControlSet\Services\DiagTrack"),
        w!("Start"),
        4,
    )?;

    // Disable "Diagnostic Policy Service"
    // Logs tons of information to be sent off and analyzed by Microsoft, and in some cases caused noticeable performance slowdown.
    set_dword_gpo(hklm, w!(r"SYSTEM\CurrentControlSet\Services\DPS"), w!("Start"), 4)?;

    // Disable "Customer Experience Improvement Program"; also implies turning off the Inventory Collector.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\AppV\CEIP"),
        w!("CEIPEnable"),
        0,
    )?;
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Microsoft\SQMClient\Windows"),
        w!("CEIPEnable"),
        0,
    )?;

    //--START-- Disable "Windows Error Reporting" service
    let wer = w!(r"SOFTWARE\Microsoft\Windows\Windows Error Reporting");
    set_dword_gpo(hklm, wer, w!("Disabled"), 1)?;
    set_dword_gpo(hklm, wer, w!("AutoApproveOSDumps"), 0)?;
    set_dword_gpo(hklm, wer, w!("DefaultConsent"), 1)?;
    set_dword_gpo(hklm, wer, w!("DefaultOverrideBehavior"), 0)?;
    set_dword_gpo(hklm, wer, w!("DontSendAdditionalData"), 1)?;
    set_dword_gpo(hklm, wer, w!("LoggingDisabled"), 1)?;

    let error_reporting = w!(r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting");
    set_dword_gpo(hklm, error_reporting, w!("AllOrNone"), 0)?;
    set_dword_gpo(hklm, error_reporting, w!("IncludeKernelFaults"), 0)?;
    set_dword_gpo(hklm, error_reporting, w!("IncludeMicrosoftApps"), 0)?;
    set_dword_gpo(hklm, error_reporting, w!("IncludeShutdownErrs"), 0)?;
    set_dword_gpo(hklm, error_reporting, w!("IncludeWindowsApps"), 0)?;
    set_dword_gpo(hklm, error_reporting, w!("DoReport"), 0)?;

    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\DeviceInstall\Settings"),
        w!("DisableSendGenericDriverNotFoundToWER"),
        1,
    )?;
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\DeviceInstall\Settings"),
        w!("DisableSendRequestAdditionalSoftwareToWER"),
        1,
    )?;

    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting"),
        w!("Disabled"),
        1,
    )?;

    set_string_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\Windows Error Reporting\ExcludedApplications"),
        w!("*"),
        w!("0"),
    )?;
    set_string_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting\ExclusionList"),
        w!("*"),
        w!("0"),
    )?;

    set_dword_gpo(
        hklm,
        w!(r"SYSTEM\CurrentControlSet\Services\WerSvc"),
        w!("Start"),
        4,
    )?;

    remove_subkey(
        &hklm_safe,
        r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting\InclusionList",
    )?;
    remove_subkey(
        &hklm_safe,
        r"SOFTWARE\Policies\Microsoft\PCHealth\ErrorReporting\Consent",
    )?;
    //--END-- Disable "Windows Error Reporting" service

    // Disable telemetry for Tablet PC's handwriting recognition
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\TabletPC"),
        w!("PreventHandwritingDataSharing"),
        1,
    )?;

    // Disable feedback reminders.
    set_dword_gpo(
        hklm,
        w!(r"Software\Policies\Microsoft\Windows\DataCollection"),
        w!("DoNotShowFeedbackNotifications"),
        1,
    )?;
    set_dword_gpo(
        hkcu,
        w!(r"SOFTWARE\Microsoft\Siuf\Rules"),
        w!("NumberOfSIUFInPeriod"),
        0,
    )?;
    set_dword_gpo(
        hkcu,
        w!(r"SOFTWARE\Microsoft\Siuf\Rules"),
        w!("PeriodInNanoSeconds"),
        0,
    )?;

    // Ask OneDrive to only generate network traffic if signed in to OneDrive
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Microsoft\OneDrive"),
        w!("PreventNetworkTrafficPreUserSignIn"),
        1,
    )?;

    // Don't ask to change the current privacy settings after applying a major
    // Windows update
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\OOBE"),
        w!("DisablePrivacyExperience"),
        1,
    )?;

    set_dword_gpo(
        hkcu,
        w!(r"Software\Microsoft\Windows\CurrentVersion\AdvertisingInfo"),
        w!("Enabled"),
        0,
    )?;
    set_dword_gpo(
        hkcu,
        w!(r"Software\Microsoft\Speech_OneCore\Settings\OnlineSpeechPrivacy"),
        w!("HasAccepted"),
        0,
    )?;
    set_dword_gpo(
        hkcu,
        w!(r"Software\Microsoft\InputPersonalization"),
        w!("RestrictImplicitInkCollection"),
        1,
    )?;
    set_dword_gpo(
        hkcu,
        w!(r"Software\Microsoft\InputPersonalization"),
        w!("RestrictImplicitTextCollection"),
        1,
    )?;
    // Don't send Microsoft inking and typing data to "improve suggestions".
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\TextInput"),
        w!("AllowLinguisticDataCollection"),
        0,
    )?;

    // Disable SmartScreen's Enhanced Phishing Protection; it's akin to Microsoft's Recall functionality.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\WTDS\Components"),
        w!("ServiceEnabled"),
        0,
    )?;

    // Disable the language list fingerprinting method; useful for bypassing geolocation restrictions.
    set_dword_gpo(
        hkcu,
        w!(r"Control Panel\International\User Profile"),
        w!("HttpAcceptLanguageOptOut"),
        1,
    )?;

    // Enable multiple processes for explorer.exe for increased stability and performance.
    set_dword_gpo(
        hkcu,
        w!(r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"),
        w!("SeparateProcess"),
        1,
    )?;

    // Hidden file extensions are abused to hide the real file format, example:
    // an executable (.exe, .scr, .com) pretending to be a PDF.
    set_dword_gpo(
        hkcu,
        w!(r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"),
        w!("HideFileExt"),
        0,
    )?;

    // Allow usage of some .appx/.appxbundle apps without a Microsoft account.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System"),
        w!("MSAOptional"),
        1
    )?;

    // Random disconnection fix for specific network adapters, such as Intel's I225-V.
    Command::new("powershell.exe")
    .args([
        "-Command",
        "Set-NetAdapterAdvancedProperty -Name '*' -DisplayName 'Wait for Link' -RegistryValue 0"
    ]).output().expect("Setting network adapter advanced property failed");

    save_registry_gpo(hklm, gpo_hklm)?;
    save_registry_gpo(hkcu, gpo_hkcu)?;
    Ok(())
}
