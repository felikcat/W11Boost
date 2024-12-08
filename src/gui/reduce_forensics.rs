use std::error::Error;
use std::process::Command;
use windows::{core::w, Win32::System::{GroupPolicy::IGroupPolicyObject, Registry::{HKEY, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE}}};

use crate::common::*;

/* Ignored for security reasons:
    - PowerShell module logging
    - Event Viewer
*/

pub fn run() -> Result<(), Box<dyn Error>> {
    let (hklm, gpo_hklm): (HKEY, IGroupPolicyObject) = init_registry_gpo(HKEY_LOCAL_MACHINE)?;
    let (hkcu, gpo_hkcu): (HKEY, IGroupPolicyObject) = init_registry_gpo(HKEY_CURRENT_USER)?;

    // Do not analyze apps' execution time data.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Perflib"),
        w!("Disable Performance Counters"),
        1,
    )?;

    // Do not keep track of recently opened files.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer"),
        w!("NoRecentDocsHistory"),
        1,
    )?;

    // Disable Superfetch.
    set_dword_gpo(
        hklm,
        w!(r"SYSTEM\CurrentControlSet\Services\SysMain"),
        w!("Start"),
        4,
    )?;
    set_dword_gpo(
        hklm,
        w!(r"SYSTEM\CurrentControlSet\Control\Session Manager\Management"),
        w!("EnableSuperfetch"),
        0,
    )?;

    // Disable Prefetch.
    set_dword_gpo(
        hklm,
        w!(r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\PrefetchParameters"),
        w!("EnablePrefetcher"),
        0,
    )?;

    // Do not use "Last Access Time Stamp Updates" by default; apps can still
    // explicitly update these timestamps for themself.
    Command::new("fsutil.exe")
        .args(["behavior", "set", "disablelastaccess", "3"])
        .output()
        .expect("fsutil.exe failed to execute");

    // Disable "Application Impact Telemetry".
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\AppCompat"),
        w!("AITEnable"),
        0,
    )?;

    // Disable "Program Compatibility Assistant".
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\AppCompat"),
        w!("DisablePCA"),
        1,
    )?;

    // Disable "Application Compatibility Engine".
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\AppCompat"),
        w!("DisableEngine"),
        1,
    )?;

    // Disable "SwitchBack Compatibility Engine".
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\AppCompat"),
        w!("SbEnable"),
        0,
    )?;

    // Disable "User Steps Recorder".
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\AppCompat"),
        w!("DisableUAR"),
        1,
    )?;

    // Disable "Inventory Collector".
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\AppCompat"),
        w!("DisableInventory"),
        1,
    )?;

    // API Sampling monitors the sampled collection of application programming interfaces used during system runtime to help diagnose compatibility problems.
    set_dword_gpo(
        hkcu,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\AppCompat"),
        w!("DisableAPISamping"),
        1,
    )?;

    // Application Footprint monitors the sampled collection of registry and file usage to help diagnose compatibility problems.
    set_dword_gpo(
        hkcu,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\AppCompat"),
        w!("DisableApplicationFootprint"),
        1,
    )?;

    // Install Tracing is a mechanism that tracks application installs to help diagnose compatibility problems.
    set_dword_gpo(
        hkcu,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\AppCompat"),
        w!("DisableInstallTracing"),
        1,
    )?;

    //The compatibility scan for backed up applications evaluates for compatibility problems in installed applications.
    set_dword_gpo(
        hkcu,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\AppCompat"),
        w!("DisableWin32AppBackup"),
        1,
    )?;

    // Disable "Program Compatibility Assistant" service.
    set_dword_gpo(hklm, w!(r"SYSTEM\CurrentControlSet\Services"), w!("PcaSvc"), 4)?;

    // Disable PerfTrack.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\WDI\{9c5a40da-b965-4fc3-8781-88dd50a6299d}"),
        w!("ScenarioExecutionEnabled"),
        0,
    )?;
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Messenger\Client"),
        w!("CEIP"),
        2,
    )?;

    // Disable tracking of application startups.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\EdgeUI"),
        w!("DisableMFUTracking"),
        1,
    )?;
    set_dword_gpo(
        hkcu,
        w!(r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"),
        w!("Start_TrackProgs"),
        0,
    )?;
    set_dword_gpo(
        hkcu,
        w!(r"Software\Policies\Microsoft\Windows\EdgeUI"),
        w!("DisableMFUTracking"),
        1,
    )?;

    // Fully disable the activity feed.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\System"),
        w!("EnableActivityFeed"),
        0,
    )?;
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\System"),
        w!("PublishUserActivities"),
        0,
    )?;
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\System"),
        w!("UploadUserActivities"),
        0,
    )?;

    // Do not search disks to attempt fixing a missing shortcut.
    set_dword_gpo(
        hkcu,
        w!(r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer"),
        w!("LinkResolveIgnoreLinkInfo"),
        1,
    )?;
    set_dword_gpo(
        hkcu,
        w!(r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer"),
        w!("NoResolveSearch"),
        1,
    )?;
    set_dword_gpo(
        hkcu,
        w!(r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer"),
        w!("NoResolveTrack"),
        1,
    )?;

    // Disable Device Search History.
    set_dword_gpo(
        hkcu,
        w!(r"Software\Microsoft\Windows\CurrentVersion\SearchSettings"),
        w!("IsDeviceSearchHistoryEnabled"),
        0,
    )?;

    // By default Windows does not automatically back-up the registry, but just in case they change this..
    set_dword_gpo(
        hkcu,
        w!(r"SYSTEM\CurrentControlSet\Control\Session Manager\Configuration Manager"),
        w!("EnablePeriodicBackup"),
        0,
    )?;

    // Remove Recommended section from Start Menu.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\Explorer"),
        w!("HideRecommendedSection"),
        1,
    )?;

    // Remove frequent programs list from the Start Menu.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\Explorer"),
        w!("NoStartMenuMFUprogramsList"),
        1,
    )?;

    // Disable File History.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\FileHistory"),
        w!("Disabled"),
        1,
    )?;

    save_registry_gpo(hklm, gpo_hklm)?;
    save_registry_gpo(hkcu, gpo_hkcu)?;

    Ok(())
}
