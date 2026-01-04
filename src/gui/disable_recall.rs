use anyhow::Result;
use winsafe::HKEY;

use w11boost::set_dword;

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
