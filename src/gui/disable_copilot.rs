use winsafe::{HKEY, prelude::advapi_Hkey as _};
use anyhow::Result;

use crate::common::set_dword;

pub fn run() -> Result<()>
{
        let hklm = HKEY::LOCAL_MACHINE;

        set_dword(
                &hklm,
                r"SOFTWARE\Policies\Microsoft\Windows\WindowsCopilot",
                "TurnOffWindowsCopilot",
                1,
        )?;

        Ok(())
}
