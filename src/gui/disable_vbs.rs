use std::error::Error;
use windows::{core::w, Win32::System::{GroupPolicy::IGroupPolicyObject, Registry::{HKEY, HKEY_LOCAL_MACHINE}}};

use crate::common::*;

pub fn run() -> Result<(), Box<dyn Error>> {
    let (hklm, gpo_hklm): (HKEY, IGroupPolicyObject) = init_registry_gpo(HKEY_LOCAL_MACHINE)?;
    
    set_string_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\DeviceGuard"),
        w!("**del.ConfigureKernelShadowStacksLaunch"),
        w!(""),
    )?;
    
    set_string_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\DeviceGuard"),
        w!("**del.ConfigureSystemGuardLaunch"),
        w!(""),
    )?;

    set_string_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\DeviceGuard"),
        w!("**del.HypervisorEnforcedCodeIntegrity"),
        w!(""),
    )?;

    set_string_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\DeviceGuard"),
        w!("**del.LsaCfgFlags"),
        w!(""),
    )?;

    set_string_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\DeviceGuard"),
        w!("**del.MachineIdentityIsolation"),
        w!(""),
    )?;

    set_string_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\DeviceGuard"),
        w!("**del.RequirePlatformSecurityFeatures"),
        w!(""),
    )?;

    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\DeviceGuard"),
        w!("EnableVirtualizationBasedSecurity"),
        0,
    )?;

    set_dword_gpo(
        hklm,
        w!(r"SOFTWARE\Policies\Microsoft\Windows\DeviceGuard"),
        w!("HVCIMATRequired"),
        0,
    )?;

    save_registry_gpo(hklm, gpo_hklm)?;

    Ok(())
}