use std::error::Error;
use windows::{core::w, Win32::System::{GroupPolicy::IGroupPolicyObject, Registry::{HKEY, HKEY_LOCAL_MACHINE}}};
use crate::common::*;

pub fn run() -> Result<(), Box<dyn Error>> {
    let (hklm, gpo_hklm): (HKEY, IGroupPolicyObject) = init_registry_gpo(HKEY_LOCAL_MACHINE)?;

    // Globally disable hibernation.
    set_dword_gpo(
        hklm,
        w!(r"SYSTEM\CurrentControlSet\Control\Power"),
        w!("HibernateEnabledDefault"),
        0,
    )?;

    // Turn off hybrid sleep (on battery).
    set_dword_gpo(
        hklm,
        w!(r"Software\Policies\Microsoft\Power\PowerSettings\94ac6d29-73ce-41a6-809f-6363ba21b47e"),
        w!("DCSettingIndex"),
        0,
    )?;

    // Turn off hybrid sleep (plugged in).
    set_dword_gpo(
        hklm,
        w!(r"Software\Policies\Microsoft\Power\PowerSettings\94ac6d29-73ce-41a6-809f-6363ba21b47e"),
        w!("ACSettingIndex"),
        0,
    )?;

    // Never idle to sleep (on battery).
    set_dword_gpo(
        hklm,
        w!(r"Software\Policies\Microsoft\Power\PowerSettings\29F6C1DB-86DA-48C5-9FDB-F2B67B1F44DA"),
        w!("DCSettingIndex"),
        0,
    )?;

    // Never idle to sleep (plugged in).
    set_dword_gpo(
        hklm,
        w!(r"Software\Policies\Microsoft\Power\PowerSettings\29F6C1DB-86DA-48C5-9FDB-F2B67B1F44DA"),
        w!("ACSettingIndex"),
        0,
    )?;

    // Never idle to hibernate (on battery).
    set_dword_gpo(
        hklm,
        w!(r"Software\Policies\Microsoft\Power\PowerSettings\9D7815A6-7EE4-497E-8888-515A05F02364"),
        w!("DCSettingIndex"),
        0,
    )?;

    // Never idle to hibernate (plugged in).
    set_dword_gpo(
        hklm,
        w!(r"Software\Policies\Microsoft\Power\PowerSettings\9D7815A6-7EE4-497E-8888-515A05F02364"),
        w!("ACSettingIndex"),
        0,
    )?;

    // Never unattended idle to sleep (on battery).
    set_dword_gpo(
        hklm,
        w!(r"Software\Policies\Microsoft\Power\PowerSettings\7bc4a2f9-d8fc-4469-b07b-33eb785aaca0"),
        w!("DCSettingIndex"),
        0,
    )?;

    // Never unattended idle to sleep (plugged in).
    set_dword_gpo(
        hklm,
        w!(r"Software\Policies\Microsoft\Power\PowerSettings\7bc4a2f9-d8fc-4469-b07b-33eb785aaca0"),
        w!("ACSettingIndex"),
        0,
    )?;

    // Disable the Hibernate entry in the Power Menu.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\FlyoutMenuSettings"),
        w!("ShowHibernateOption"),
        0,
    )?;

    // Disable the Sleep entry in the Power Menu.
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\FlyoutMenuSettings"),
        w!("ShowSleepOption"),
        0,
    )?;

    save_registry_gpo(hklm, gpo_hklm)?;

    Ok(())
}
