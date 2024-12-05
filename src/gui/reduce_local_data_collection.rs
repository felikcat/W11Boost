use std::error::Error;
use std::process::Command;
use winsafe::{HKEY, prelude::advapi_Hkey};

use crate::common::*;

pub fn run() -> Result<(), Box<dyn Error>> {
    let hklm = HKEY::LOCAL_MACHINE;
    let hkcu = HKEY::CURRENT_USER;

    // Do not analyze apps' execution time data.
    set_dword(
        &hklm,
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Perflib",
        "Disable Performance Counters",
        1,
    )?;

    // Do not keep track of recently opened files.
    set_dword(
        &hklm,
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
        "NoRecentDocsHistory",
        1,
    )?;

    // Disable Superfetch.
    set_dword(
        &hklm,
        r"SYSTEM\CurrentControlSet\Services\SysMain",
        "Start",
        4,
    )?;
    set_dword(
        &hklm,
        r"SYSTEM\CurrentControlSet\Control\Session Manager\Management",
        "EnableSuperfetch",
        0,
    )?;

    // Disable Prefetch.
    set_dword(
        &hklm,
        r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\PrefetchParameters",
        "EnablePrefetcher",
        0,
    )?;

    // Do not use "Last Access Time Stamp Updates" by default; apps can still
    // explicitly update these timestamps for themself.
    Command::new("fsutil.exe")
        .args(["behavior", "set", "disablelastaccess", "3"])
        .output()
        .expect("fsutil.exe failed to execute");

    // Disable "Application Impact Telemetry".
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
        "AITEnable",
        0,
    )?;

    // Disable "Program Compatibility Assistant".
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
        "DisablePCA",
        1,
    )?;

    // Disable "Application Compatibility Engine".
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
        "DisableEngine",
        1,
    )?;

    // Disable "SwitchBack Compatibility Engine".
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
        "SbEnable",
        0,
    )?;

    // Disable "User Steps Recorder".
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
        "DisableUAR",
        1,
    )?;

    // Disable "Inventory Collector".
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
        "DisableInventory",
        1,
    )?;

    // Disable "Program Compatibility Assistant" service.
    set_dword(&hklm, r"SYSTEM\CurrentControlSet\Services", "PcaSvc", 4)?;

    // Disable PerfTrack.
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows\WDI\{9c5a40da-b965-4fc3-8781-88dd50a6299d}",
        "ScenarioExecutionEnabled",
        0,
    )?;
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Messenger\Client",
        "CEIP",
        2,
    )?;

    // Disable tracking of application startups.
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows\EdgeUI",
        "DisableMFUTracking",
        1,
    )?;
    set_dword(
        &hkcu,
        r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
        "Start_TrackProgs",
        0,
    )?;
    set_dword(
        &hkcu,
        r"Software\Policies\Microsoft\Windows\EdgeUI",
        "DisableMFUTracking",
        1,
    )?;

    // Fully disable the activity feed.
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows\System",
        "EnableActivityFeed",
        0,
    )?;
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows\System",
        "PublishUserActivities",
        0,
    )?;
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows\System",
        "UploadUserActivities",
        0,
    )?;

    // Do not search disks to attempt fixing a missing shortcut.
    set_dword(
        &hkcu,
        r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
        "LinkResolveIgnoreLinkInfo",
        1,
    )?;
    set_dword(
        &hkcu,
        r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
        "NoResolveSearch",
        1,
    )?;
    set_dword(
        &hkcu,
        r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer",
        "NoResolveTrack",
        1,
    )?;

    // Disable Device Search History.
    set_dword(
        &hkcu,
        r"Software\Microsoft\Windows\CurrentVersion\SearchSettings",
        "IsDeviceSearchHistoryEnabled",
        0,
    )?;

    // By default Windows does not automatically back-up the registry, but just in case they change this..
    set_dword(
        &hkcu,
        r"SYSTEM\CurrentControlSet\Control\Session Manager\Configuration Manager",
        "EnablePeriodicBackup",
        0,
    )?;

    // Remove Recommended section from Start Menu.
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
        "HideRecommendedSection",
        1,
    )?;

    // Remove frequent programs list from the Start Menu.
    set_dword(
        &hklm,
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer",
        "NoStartMenuMFUprogramsList",
        1,
    )?;

    Ok(())
}
