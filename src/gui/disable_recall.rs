use winsafe::{HKEY, prelude::advapi_Hkey};

use crate::common::*;

pub fn run() -> anyhow::Result<()>
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
