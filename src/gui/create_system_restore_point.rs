use crate::common::*;
use std::error::Error;
use windows::Win32::System::Restore::*;
use winsafe::{self, HKEY, prelude::*};

fn create_restore_point() -> Result<(), Box<dyn Error>> {
    let mut smgr_status: STATEMGRSTATUS = Default::default();

    let description: Vec<u16> = "Installed W11Boost".encode_utf16().collect();
    let mut sz_description = [0; 256];
    sz_description[..description.len()].copy_from_slice(&description);

    let mut restore_point_info = RESTOREPOINTINFOW {
        dwEventType: BEGIN_SYSTEM_CHANGE,
        dwRestorePtType: APPLICATION_INSTALL,
        llSequenceNumber: 0,
        szDescription: sz_description,
    };

    unsafe {
        // Start System Restore point.
        SRSetRestorePointW(&restore_point_info, &mut smgr_status)
            .expect("SRSetRestorePointW 'start' failed");
    }

    restore_point_info.dwEventType = END_SYSTEM_CHANGE;
    restore_point_info.llSequenceNumber = smgr_status.llSequenceNumber;

    unsafe {
        // End System Restore point.
        SRSetRestorePointW(&restore_point_info, &mut smgr_status)
            .expect("SRSetRestorePointW 'end' failed");
    }

    Ok(())
}

fn restore_point_prep() -> Result<(), Box<dyn Error>> {
    let hklm = HKEY::LOCAL_MACHINE;
    set_dword(
        &hklm,
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\SystemRestore",
        "SystemRestorePointCreationFrequency",
        0,
    )?;

    remove_subkey(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows NT\SystemRestore\DisableConfig",
    )?;

    remove_subkey(
        &hklm,
        r"SOFTWARE\Policies\Microsoft\Windows NT\SystemRestore\DisableSR",
    )?;

    Ok(())
}

fn revert_restore_point_prep() -> Result<(), Box<dyn Error>> {
    let hklm = HKEY::LOCAL_MACHINE;
    remove_subkey(&hklm, "SystemRestorePointCreationFrequency")?;
    Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    restore_point_prep()?;
    create_restore_point()?;
    revert_restore_point_prep()?;
    Ok(())
}
