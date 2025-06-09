use crate::common::*;
use std::error::Error;
use winsafe::{HKEY, prelude::advapi_Hkey};

pub fn run() -> Result<(), Box<dyn Error>> {
        let hklm = HKEY::LOCAL_MACHINE;
        enable_sleep(&hklm)?;
        Ok(())
}

fn enable_sleep(hklm: &HKEY) -> Result<(), Box<dyn Error>> {
        remove_subkey(&hklm, r"SYSTEM\CurrentControlSet\Control\Power\HibernateEnabledDefault")?;

        remove_subkey(
                &hklm,
                r"Software\Policies\Microsoft\Power\PowerSettings\94ac6d29-73ce-41a6-809f-6363ba21b47e\DCSettingIndex",
        )?;
        remove_subkey(
                &hklm,
                r"Software\Policies\Microsoft\Power\PowerSettings\94ac6d29-73ce-41a6-809f-6363ba21b47e\ACSettingIndex",
        )?;

        remove_subkey(
                &hklm,
                r"Software\Policies\Microsoft\Power\PowerSettings\29F6C1DB-86DA-48C5-9FDB-F2B67B1F44DA\DCSettingIndex",
        )?;
        remove_subkey(
                &hklm,
                r"Software\Policies\Microsoft\Power\PowerSettings\29F6C1DB-86DA-48C5-9FDB-F2B67B1F44DA\ACSettingIndex",
        )?;

        remove_subkey(
                &hklm,
                r"Software\Policies\Microsoft\Power\PowerSettings\9D7815A6-7EE4-497E-8888-515A05F02364\DCSettingIndex",
        )?;
        remove_subkey(
                &hklm,
                r"Software\Policies\Microsoft\Power\PowerSettings\9D7815A6-7EE4-497E-8888-515A05F02364\ACSettingIndex",
        )?;

        remove_subkey(
                &hklm,
                r"Software\Policies\Microsoft\Power\PowerSettings\7bc4a2f9-d8fc-4469-b07b-33eb785aaca0\DCSettingIndex",
        )?;
        remove_subkey(
                &hklm,
                r"Software\Policies\Microsoft\Power\PowerSettings\7bc4a2f9-d8fc-4469-b07b-33eb785aaca0\ACSettingIndex",
        )?;

        remove_subkey(
                &hklm,
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\FlyoutMenuSettings\ShowHibernateOption",
        )?;
        remove_subkey(
                hklm,
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\FlyoutMenuSettings\ShowSleepOption",
        )?;

        Ok(())
}
