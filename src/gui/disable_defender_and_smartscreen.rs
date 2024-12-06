use crate::common::*;
use fltk::dialog;
use std::error::Error;
use windows::{
    core::{w, IUnknown}, Win32::System::{
        Com::{CoCreateInstance, CoCreateInstanceEx, CLSCTX_INPROC_SERVER},
        GroupPolicy::{CLSID_GroupPolicyObject, IGroupPolicyObject, IGroupPolicyObject_Vtbl, GPO_OPEN_LOAD_REGISTRY, GPO_SECTION_USER}, Registry::{HKEY, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE},
    }
};
use winsafe::{self as ws, CoInitializeEx, co::IID, prelude::advapi_Hkey};

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut hkcu: HKEY = HKEY_CURRENT_USER;

    // The apartment thread model is required for GPOs.
    CoInitializeEx(ws::co::COINIT::APARTMENTTHREADED).expect("CoInitializeEx failed");

    unsafe {
        let gpo: IGroupPolicyObject =
            CoCreateInstance(&CLSID_GroupPolicyObject, None, CLSCTX_INPROC_SERVER)
                .expect("Failed to create GPO object");

        gpo.OpenLocalMachineGPO(GPO_OPEN_LOAD_REGISTRY)
            .expect("Failed to open local machine GPO");

        gpo.GetRegistryKey(GPO_SECTION_USER, &mut hkcu).expect("GetRegistryKey failed");
    };

    // Disable Windows SmartScreen.
    /*set_string_gpo(
        hkcu,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\System"),
        w!("**del.ShellSmartScreenLevel"),
        w!(""),
    )?;
    */

    // Disable Windows Defender; it's done excessively just to be sure.
    set_dword_gpo(
        hkcu,
        w!(r"SOFTWARE\Policies\Microsoft\Windows Defender"),
        w!("DisableAntiSpyware"),
        1,
    )?;
    set_dword_gpo(
        hkcu,
        w!(r"SOFTWARE\Policies\Microsoft\Windows Defender"),
        w!("DisableAntiVirus"),
        1,
    )?;
    set_dword_gpo(
        hkcu,
        w!(r"SOFTWARE\Policies\Microsoft\Windows Defender"),
        w!("ServiceKeepAlive"),
        0,
    )?;
    set_dword_gpo(
        hkcu,
        w!(r"SOFTWARE\Policies\Microsoft\Windows Defender\Real-Time Protection"),
        w!("DisableRealtimeMonitoring"),
        1,
    )?;
    set_dword_gpo(
        hkcu,
        w!(r"SOFTWARE\Policies\Microsoft\Windows Defender\Real-Time Protection"),
        w!("DisableBehaviorMonitoring"),
        1,
    )?;
    set_dword_gpo(
        hkcu,
        w!(r"SOFTWARE\Policies\Microsoft\Windows Defender\Real-Time Protection"),
        w!("DisableIOAVProtection"),
        1,
    )?;
    set_dword_gpo(
        hkcu,
        w!(r"SOFTWARE\Policies\Microsoft\Windows Defender\Real-Time Protection"),
        w!("DisableOnAccessProtection"),
        1,
    )?;
    set_dword_gpo(
        hkcu,
        w!(r"SOFTWARE\Policies\Microsoft\Windows Defender\Real-Time Protection"),
        w!("DisableScriptScanning"),
        1,
    )?;

    Ok(())
}
