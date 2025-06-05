use chrono::{Datelike, Timelike, Utc};
use fltk::app;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::os::raw::c_void;
use std::path::PathBuf;
use std::ptr::null_mut;
use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::System::Registry::{REG_DWORD, REG_SZ, RegCreateKeyW, RegSetValueExW};
use windows::core::PCWSTR;
use windows::{
        Win32::System::{
                Com::{CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx},
                GroupPolicy::{CLSID_GroupPolicyObject, GPO_OPEN_LOAD_REGISTRY, GPO_SECTION_MACHINE, IGroupPolicyObject, REGISTRY_EXTENSION_GUID},
                Registry::RegCloseKey,
        },
        core::GUID,
};
use winsafe::{
        self as w, HKEY, RegistryValue,
        co::{self, KNOWNFOLDERID},
        prelude::*,
};

pub fn get_windows_path(folder_id: &KNOWNFOLDERID) -> Result<String, Box<dyn Error>>
{
        let the_path = w::SHGetKnownFolderPath(folder_id, co::KF::DEFAULT, None)?;
        Ok(the_path)
}

pub fn set_dword_gpo(hkey: windows::Win32::System::Registry::HKEY, subkey: PCWSTR, value_name: PCWSTR, value: u32) -> Result<(), Box<dyn Error>>
{
        unsafe {
                let mut new_key: windows::Win32::System::Registry::HKEY = hkey;

                let result = RegCreateKeyW(new_key, subkey, &mut new_key);
                if result != ERROR_SUCCESS {
                        return Err(format!("[set_dword_gpo] Failed to create key: {:?}", result).into());
                }

                let bytes = value.to_ne_bytes();

                let result = RegSetValueExW(new_key, value_name, Some(0), REG_DWORD, Some(&bytes));
                if result != ERROR_SUCCESS {
                        return Err(format!("[set_dword_gpo] Failed to set key value: {:?}", result).into());
                }

                let result = RegCloseKey(new_key);
                if result != ERROR_SUCCESS {
                        return Err(format!("[set_dword_gpo] Failed to close key: {:?}", result).into());
                }
        }
        Ok(())
}

pub fn set_string_gpo(hkey: windows::Win32::System::Registry::HKEY, subkey: PCWSTR, value_name: PCWSTR, value: PCWSTR) -> Result<(), Box<dyn Error>>
{
        unsafe {
                let mut new_key: windows::Win32::System::Registry::HKEY = hkey;

                let result = RegCreateKeyW(new_key, subkey, &mut new_key);
                if result != ERROR_SUCCESS {
                        return Err(format!("[set_string_gpo] Failed to create key: {:?}", result).into());
                }

                let bytes = value.as_wide();
                let length = bytes.len().checked_mul(2).unwrap();
                let bytes_cast: *const u8 = bytes.as_ptr().cast();
                let slice = std::slice::from_raw_parts(bytes_cast, length);

                let result = RegSetValueExW(new_key, value_name, Some(0), REG_SZ, Some(slice));
                if result.is_err() {
                        return Err(format!("[set_string_gpo] Failed to set key: {:?}", result).into());
                }

                let result = RegCloseKey(new_key);
                if result.is_err() {
                        return Err(format!("[set_string_gpo] Failed to close key: {:?}", result).into());
                }
        }
        Ok(())
}

pub fn set_dword(hkey: &HKEY, subkey: &str, value_name: &str, value: u32) -> Result<(), Box<dyn Error>>
{
        let o_subkey = subkey;
        let hkey_text = get_hkey_text(hkey)?;

        let (subkey, _) = hkey
                .RegCreateKeyEx(subkey, None, co::REG_OPTION::NON_VOLATILE, co::KEY::WRITE, None)
                .map_err(|e| format!("Failed to open a DWORD in key: {}\\{}\\{} - Error: {}", hkey_text, o_subkey, value_name, e))?;

        subkey.RegSetValueEx(Some(value_name), RegistryValue::Dword(value))
                .map_err(|e| format!("Failed to set DWORD value in key: {}\\{}\\{} - Error: {}", hkey_text, o_subkey, value_name, e))?;

        match log_registry(hkey, o_subkey, value_name, &value.to_string(), "DWORD") {
                Ok(_) => Ok(()),
                Err(e) => Err(format!(
                        "Failed to log DWORD change for key: {}\\{}\\{} -> {} -> Error: {}",
                        hkey_text, o_subkey, value_name, value, e
                )),
        }?;

    Ok(())
}

pub fn set_string(hkey: &HKEY, subkey: &str, value_name: &str, value: &str) -> Result<(), Box<dyn Error>>
{
        let o_subkey = subkey;
        let hkey_text = get_hkey_text(hkey).unwrap();

        let (subkey, _) = hkey
                .RegCreateKeyEx(subkey, None, co::REG_OPTION::NON_VOLATILE, co::KEY::WRITE, None)
                .map_err(|e| format!("Failed to open a Sz in key: {}\\{}\\{} - Error: {}", hkey_text, o_subkey, value_name, e))?;

        let value = value.to_string();
        subkey.RegSetValueEx(Some(value_name), RegistryValue::Sz(value.clone()))
                .map_err(|e| format!("Failed to set Sz value in key: {}\\{}\\{} - Error: {}", hkey_text, o_subkey, value_name, e))?;

        match log_registry(hkey, o_subkey, value_name, &value, "String") {
                Ok(_) => Ok(()),
                Err(e) => Err(format!(
                        "Failed to log Sz change for key: {}\\{}\\{} -> {} -> Error: {}",
                        hkey_text, o_subkey, value_name, value, e
                )),
        }?;

        Ok(())
}

pub fn remove_subkey(hkey: &HKEY, subkey: &str) -> Result<(), Box<dyn Error>>
{
        let o_subkey = subkey;
        let hkey_text = get_hkey_text(hkey).unwrap();

        match hkey.RegDeleteTree(Some(subkey)) {
                Ok(_) => Ok(()),
                Err(e) if e == w::co::ERROR::FILE_NOT_FOUND => Ok(()),
                Err(e) => Err(Box::new(e)),
        }
        .map_err(|e| format!("Failed to delete subkey: {}\\{} - Error: {}", hkey_text, o_subkey, e))?;

        match log_registry(hkey, o_subkey, "->", "", "Removed") {
            Ok(_) => Ok(()),
            Err(e) => Err(format!(
                "Failed to log removal of key: {}\\{} - Error: {}", hkey_text, o_subkey, e
            ))
        }?;
        log_registry(hkey, o_subkey, "->", "", "Removed")?;
        Ok(())
}

pub fn check_dword(hkey: &HKEY, subkey: &str, value_name: &str, expected_value: u32) -> Result<bool, Box<dyn Error>>
{
        let o_subkey = subkey;
        let hkey_text = get_hkey_text(hkey).unwrap();

        let subkey = match hkey.RegGetValue(Some(subkey), Some(value_name), co::RRF::RT_DWORD) {
                Ok(value) => value,
                Err(e) if e == w::co::ERROR::FILE_NOT_FOUND => return Ok(false),
                Err(e) => {
                        return Err(format!(
                                "Failed to open key for DWORD check: {}\\{}\\{} -> Error: {}",
                                hkey_text, o_subkey, value_name, e
                        )
                        .into());
                }
        };

        if let RegistryValue::Dword(value) = subkey {
                if value != expected_value { Ok(false) } else { Ok(true) }
        } else {
                Err("Expected DWORD value".into())
        }
}

fn get_hkey_text(hkey: &HKEY) -> Result<&str, Box<dyn Error>>
{
        let result = if *hkey == HKEY::LOCAL_MACHINE {
                "HKEY_LOCAL_MACHINE"
        } else if *hkey == HKEY::CURRENT_USER {
                "HKEY_CURRENT_USER"
        } else {
                "UNKNOWN_HKEY"
        };

        Ok(result)
}

fn log_registry(hkey: &HKEY, subkey: &str, value_name: &str, value: &str, type_name: &str) -> Result<(), Box<dyn Error>>
{
        let hkey_text = get_hkey_text(hkey)?;

        // Can't use &KNOWNFOLDERID::Desktop because we're running as TrustedInstaller.
        let desktop_dir = get_windows_path(&KNOWNFOLDERID::PublicDesktop)?;
        let mut log_path = PathBuf::from(desktop_dir);
        log_path.push("W11Boost Logs");

        if !log_path.exists() {
                fs::create_dir_all(&log_path).map_err(|e| format!("Failed to create log directory: {} - {}", log_path.display(), e))?;
        }

        let now = Utc::now();
        let time_info = format!(
                "{:02}/{:02}/{} {:02}:{:02}:{:02}",
                now.day(),
                now.month(),
                now.year(),
                now.hour(),
                now.minute(),
                now.second()
        );

        let log_entry = format!("{} -> {}\\{}\\{}\\{} -> {}\n", time_info, hkey_text, subkey, value_name, type_name, value);

        log_path.push("Registry.log");

        let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&log_path)
                .map_err(|e| format!("Failed to open/create log file: {} - {}", log_path.display(), e))?;

        // Write log entry with explicit error handling
        file.write_all(log_entry.as_bytes())
                .map_err(|e| format!("Failed to write to log file: {} - {}", log_path.display(), e))?;

        Ok(())
}

pub fn center() -> (i32, i32)
{
        ((app::screen_size().0 / 2.0) as i32, (app::screen_size().1 / 2.0) as i32)
}

pub fn init_registry_gpo(
        mut hkey: windows::Win32::System::Registry::HKEY,
) -> Result<(windows::Win32::System::Registry::HKEY, IGroupPolicyObject), Box<dyn Error>>
{
        unsafe {
                // The apartment thread model is required for GPOs.
                let result = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
                if result.is_err() {
                        return Err(format!("Failed to run CoInitalizeEx: {:?}", result).into());
                }
                let gpo: IGroupPolicyObject = CoCreateInstance(&CLSID_GroupPolicyObject, None, CLSCTX_INPROC_SERVER).expect("Failed to create GPO object");

                gpo.OpenLocalMachineGPO(GPO_OPEN_LOAD_REGISTRY).expect("Failed to open local machine GPO");

                gpo.GetRegistryKey(GPO_SECTION_MACHINE, &mut hkey).expect("GetRegistryKey failed");

                Ok((hkey, gpo))
        }
}

pub fn save_registry_gpo(hkey: windows::Win32::System::Registry::HKEY, gpo: IGroupPolicyObject) -> Result<(), Box<dyn Error>>
{
        let mut snap_guid = GUID::from_u128(0x0f6b957e_509e_11d1_a7cc_0000f87571e3);
        let mut registry_guid = REGISTRY_EXTENSION_GUID;
        unsafe {
                gpo.Save(true.into(), false.into(), &mut registry_guid, &mut snap_guid)
                        .expect("Failed to save GPO changes");
        }

        let result = unsafe { RegCloseKey(hkey) };
        if result.is_err() {
                eprintln!("RegCloseKey failed");
        }

        Ok(())
}
