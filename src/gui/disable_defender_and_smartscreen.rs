use crate::common::*;
use fltk::dialog;
use windows::{core::w, Win32::System::{GroupPolicy::IGroupPolicyObject, Registry::{HKEY, HKEY_LOCAL_MACHINE}}};
use std::error::Error;
use winsafe::{HKEY as HKEY_SAFE, prelude::advapi_Hkey};

pub fn run() -> Result<(), Box<dyn Error>> {
    let (hklm, gpo_hklm): (HKEY, IGroupPolicyObject) = init_registry_gpo(HKEY_LOCAL_MACHINE)?;
    
    let hklm_safe = HKEY_SAFE::LOCAL_MACHINE;
    let tamper_disabled = check_dword(
        &hklm_safe,
        r"SOFTWARE\Microsoft\Windows Defender\Features",
        "TamperProtection",
        4,
    )?;
    let realtime_disabled = check_dword(
        &hklm_safe,
        r"SOFTWARE\Microsoft\Windows Defender\Real-Time Protection",
        "DisableRealtimeMonitoring",
        1,
    )?;

    // Not necessary, but it will be later.
    if !tamper_disabled || !realtime_disabled {
        dialog::alert(
            center().0,
            center().1,
            "Windows Defender's \"Tamper Protection\" or \"Real-time protection\" is enabled.\nDisable both now, then close this dialog.",
        );
    }

    // Diasble anti-tamper.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Microsoft\Windows Defender\Features"),
        w!("TamperProtection"),
        4,
    )?;

    // Disable Windows SmartScreen.
    set_string_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\System"),
        w!("**del.ShellSmartScreenLevel"),
        w!(""),
    )?;
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\System"),
        w!("EnableSmartScreen"),
        0,
    )?;

    // Disable Windows Defender; it's done excessively just to be sure.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows Defender"),
        w!("DisableAntiSpyware"),
        1,
    )?;
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows Defender"),
        w!("DisableAntiVirus"),
        1,
    )?;
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows Defender"),
        w!("ServiceKeepAlive"),
        0,
    )?;
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows Defender\Real-Time Protection"),
        w!("DisableRealtimeMonitoring"),
        1,
    )?;
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows Defender\Real-Time Protection"),
        w!("DisableBehaviorMonitoring"),
        1,
    )?;
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows Defender\Real-Time Protection"),
        w!("DisableIOAVProtection"),
        1,
    )?;
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows Defender\Real-Time Protection"),
        w!("DisableOnAccessProtection"),
        1,
    )?;
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows Defender\Real-Time Protection"),
        w!("DisableScriptScanning"),
        1,
    )?;

    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Microsoft\Windows Defender"),
        w!("DisableAntiSpyware"),
        1,
    )?;
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Microsoft\Windows Defender"),
        w!("DisableAntiVirus"),
        1,
    )?;


    set_dword_gpo(
        hklm,
        w!(r"System\CurrentControlSet\Services\WdFilter"),
        w!("Start"),
        4,
    )?;
    set_dword_gpo(
        hklm,
        w!(r"System\CurrentControlSet\Services\WdNisDrv"),
        w!("Start"),
        4,
    )?;
    set_dword_gpo(
        hklm,
        w!(r"System\CurrentControlSet\Services\WdNisSvc"),
        w!("Start"),
        4,
    )?;
    set_dword_gpo(
        hklm,
        w!(r"System\CurrentControlSet\Services\WinDefend"),
        w!("Start"),
        4,
    )?;
    set_dword_gpo(
        hklm,
        w!(r"System\CurrentControlSet\Services\MDCoreSvc"),
        w!("Start"),
        4,
    )?;

    save_registry_gpo(hklm, gpo_hklm)?;

    Ok(())
}
