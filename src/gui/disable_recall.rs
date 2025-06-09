use winsafe::{HKEY, prelude::advapi_Hkey};

use crate::common::*;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
        let hklm = HKEY::LOCAL_MACHINE;

        set_dword(
                &hklm,
                r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
                "AllowRecallEnablement",
                0,
        )?;

        Ok(())
}
