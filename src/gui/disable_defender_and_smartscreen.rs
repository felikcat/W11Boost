use crate::common::*;
use fltk::dialog;
use std::error::Error;
use winsafe::{HKEY, prelude::advapi_Hkey};

// TODO -> Make this use Group Policy Objects instead of directly modifying the registry.

pub fn run() -> Result<(), Box<dyn Error>> {
    let hklm = HKEY::LOCAL_MACHINE;

    let tamper_disabled = check_dword(
        &hklm,
        r"SOFTWARE\Microsoft\Windows Defender\Features",
        "TamperProtection",
        4,
    )?;
    let realtime_disabled = check_dword(
        &hklm,
        r"SOFTWARE\Microsoft\Windows Defender\Real-Time Protection",
        "DisableRealtimeMonitoring",
        1,
    )?;

    if !tamper_disabled || !realtime_disabled {
        dialog::alert(
            center().0,
            center().1,
            "Windows Defender's \"Tamper Protection\" or \"Real-time protection\" is enabled.\nDisable both now, then close this dialog.",
        );
    }

    // Disable Windows SmartScreen.
    set_string(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows\System",
        "**del.ShellSmartScreenLevel",
        "",
    )?;

    // Disable Windows Defender; it's done excessively just to be sure.
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows Defender",
        "DisableAntiSpyware",
        1,
    )?;
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows Defender",
        "DisableAntiVirus",
        1,
    )?;
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows Defender",
        "ServiceKeepAlive",
        0,
    )?;
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows Defender\Real-Time Protection",
        "DisableRealtimeMonitoring",
        1,
    )?;
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows Defender\Real-Time Protection",
        "DisableBehaviorMonitoring",
        1,
    )?;
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows Defender\Real-Time Protection",
        "DisableIOAVProtection",
        1,
    )?;
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows Defender\Real-Time Protection",
        "DisableOnAccessProtection",
        1,
    )?;
    set_dword(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows Defender\Real-Time Protection",
        "DisableScriptScanning",
        1,
    )?;

    Ok(())
}
