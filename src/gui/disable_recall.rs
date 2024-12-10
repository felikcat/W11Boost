use std::error::Error;
use windows::{core::w, Win32::System::{GroupPolicy::IGroupPolicyObject, Registry::{HKEY, HKEY_LOCAL_MACHINE}}};
use crate::common::*;

pub fn run() -> Result<(), Box<dyn Error>>{
    let (hklm, gpo_hklm): (HKEY, IGroupPolicyObject) = init_registry_gpo(HKEY_LOCAL_MACHINE)?;
    
    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI"),
        w!("AllowRecallEnablement"),
        0,
    )?;

    save_registry_gpo(hklm, gpo_hklm)?;

    Ok(())
}