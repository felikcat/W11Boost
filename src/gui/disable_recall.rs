use winsafe::{HKEY, prelude::advapi_Hkey};
use anyhow::Result;

use crate::common::*;

pub fn run() -> Result<()>
{
        let hklm = HKEY::LOCAL_MACHINE;

        set_dword(
                &hklm,
                r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
                "AllowRecallEnablement",
                0,
        )?;

        Ok(())
}
