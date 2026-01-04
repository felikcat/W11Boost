use anyhow::Result;
use winsafe::HKEY;

use w11boost::set_dword;

// (hkey: "HKLM"|"HKCU", subkey, value_name, value)
const REGISTRY_DWORDS: &[(&str, &str, &str, u32)] = &[
	// Globally disable hibernation
	("HKLM", r"SYSTEM\CurrentControlSet\Control\Power", "HibernateEnabledDefault", 0),
	// Turn off hybrid sleep (on battery)
	("HKLM", r"Software\Policies\Microsoft\Power\PowerSettings\94ac6d29-73ce-41a6-809f-6363ba21b47e", "DCSettingIndex", 0),
	// Turn off hybrid sleep (plugged in)
	("HKLM", r"Software\Policies\Microsoft\Power\PowerSettings\94ac6d29-73ce-41a6-809f-6363ba21b47e", "ACSettingIndex", 0),
	// Never idle to sleep (on battery)
	("HKLM", r"Software\Policies\Microsoft\Power\PowerSettings\29F6C1DB-86DA-48C5-9FDB-F2B67B1F44DA", "DCSettingIndex", 0),
	// Never idle to sleep (plugged in)
	("HKLM", r"Software\Policies\Microsoft\Power\PowerSettings\29F6C1DB-86DA-48C5-9FDB-F2B67B1F44DA", "ACSettingIndex", 0),
	// Never idle to hibernate (on battery)
	("HKLM", r"Software\Policies\Microsoft\Power\PowerSettings\9D7815A6-7EE4-497E-8888-515A05F02364", "DCSettingIndex", 0),
	// Never idle to hibernate (plugged in)
	("HKLM", r"Software\Policies\Microsoft\Power\PowerSettings\9D7815A6-7EE4-497E-8888-515A05F02364", "ACSettingIndex", 0),
	// Never unattended idle to sleep (on battery)
	("HKLM", r"Software\Policies\Microsoft\Power\PowerSettings\7bc4a2f9-d8fc-4469-b07b-33eb785aaca0", "DCSettingIndex", 0),
	// Never unattended idle to sleep (plugged in)
	("HKLM", r"Software\Policies\Microsoft\Power\PowerSettings\7bc4a2f9-d8fc-4469-b07b-33eb785aaca0", "ACSettingIndex", 0),
	// Disable the Hibernate entry in the Power Menu
	("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\FlyoutMenuSettings", "ShowHibernateOption", 0),
	// Disable the Sleep entry in the Power Menu
	("HKLM", r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\FlyoutMenuSettings", "ShowSleepOption", 0),
];

fn get_hkey(hkey_str: &str) -> HKEY
{
	if hkey_str == "HKLM" { HKEY::LOCAL_MACHINE } else { HKEY::CURRENT_USER }
}

pub fn run() -> Result<()>
{
	for (hkey_str, subkey, value_name, value) in REGISTRY_DWORDS {
		set_dword(&get_hkey(hkey_str), subkey, value_name, *value)?;
	}
	Ok(())
}
