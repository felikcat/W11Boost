use chrono::{Datelike, Timelike, Utc};
use fltk::app;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use winsafe::{
        self as w, HKEY, RegistryValue,
        co::{self, KNOWNFOLDERID},
        prelude::*,
};

pub fn get_windows_path(folder_id: &KNOWNFOLDERID) -> Result<String, Box<dyn Error>> {
        let the_path = w::SHGetKnownFolderPath(folder_id, co::KF::DEFAULT, None)?;
        Ok(the_path)
}

pub fn set_dword(hkey: &HKEY, subkey: &str, value_name: &str, value: u32) -> Result<(), Box<dyn Error>> {
        let o_subkey = subkey;
        let hkey_text = get_hkey_text(hkey)?;

        let (subkey, _) = hkey
                .RegCreateKeyEx(subkey, None, co::REG_OPTION::NON_VOLATILE, co::KEY::WRITE, None)
                .map_err(|e| {
                        format!(
                                "Failed to open a DWORD in key: {}\\{}\\{} - Error: {}",
                                hkey_text, o_subkey, value_name, e
                        )
                })?;

        subkey.RegSetValueEx(Some(value_name), RegistryValue::Dword(value))
                .map_err(|e| {
                        format!(
                                "Failed to set DWORD value in key: {}\\{}\\{} - Error: {}",
                                hkey_text, o_subkey, value_name, e
                        )
                })?;

        match log_registry(hkey, o_subkey, value_name, &value.to_string(), "DWORD") {
                Ok(_) => Ok(()),
                Err(e) => Err(format!(
                        "Failed to log DWORD change for key: {}\\{}\\{} -> {} -> Error: {}",
                        hkey_text, o_subkey, value_name, value, e
                )),
        }?;

        Ok(())
}

pub fn set_string(hkey: &HKEY, subkey: &str, value_name: &str, value: &str) -> Result<(), Box<dyn Error>> {
        let o_subkey = subkey;
        let hkey_text = get_hkey_text(hkey).unwrap();

        let (subkey, _) = hkey
                .RegCreateKeyEx(subkey, None, co::REG_OPTION::NON_VOLATILE, co::KEY::WRITE, None)
                .map_err(|e| {
                        format!(
                                "Failed to open a Sz in key: {}\\{}\\{} - Error: {}",
                                hkey_text, o_subkey, value_name, e
                        )
                })?;

        let value = value.to_string();
        subkey.RegSetValueEx(Some(value_name), RegistryValue::Sz(value.clone()))
                .map_err(|e| {
                        format!(
                                "Failed to set Sz value in key: {}\\{}\\{} - Error: {}",
                                hkey_text, o_subkey, value_name, e
                        )
                })?;

        match log_registry(hkey, o_subkey, value_name, &value, "String") {
                Ok(_) => Ok(()),
                Err(e) => Err(format!(
                        "Failed to log Sz change for key: {}\\{}\\{} -> {} -> Error: {}",
                        hkey_text, o_subkey, value_name, value, e
                )),
        }?;

        Ok(())
}

pub fn remove_subkey(hkey: &HKEY, subkey: &str) -> Result<(), Box<dyn Error>> {
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
                        "Failed to log removal of key: {}\\{} - Error: {}",
                        hkey_text, o_subkey, e
                )),
        }?;
        log_registry(hkey, o_subkey, "->", "", "Removed")?;
        Ok(())
}

pub fn check_dword(hkey: &HKEY, subkey: &str, value_name: &str, expected_value: u32) -> Result<bool, Box<dyn Error>> {
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

fn get_hkey_text(hkey: &HKEY) -> Result<&str, Box<dyn Error>> {
        let result = if *hkey == HKEY::LOCAL_MACHINE {
                "HKEY_LOCAL_MACHINE"
        } else if *hkey == HKEY::CURRENT_USER {
                "HKEY_CURRENT_USER"
        } else {
                "UNKNOWN_HKEY"
        };

        Ok(result)
}

fn log_registry(
        hkey: &HKEY,
        subkey: &str,
        value_name: &str,
        value: &str,
        type_name: &str,
) -> Result<(), Box<dyn Error>> {
        let hkey_text = get_hkey_text(hkey)?;

        // Can't use &KNOWNFOLDERID::Desktop because we're running as TrustedInstaller.
        let desktop_dir = get_windows_path(&KNOWNFOLDERID::PublicDesktop)?;
        let mut log_path = PathBuf::from(desktop_dir);
        log_path.push("W11Boost Logs");

        if !log_path.exists() {
                fs::create_dir_all(&log_path)
                        .map_err(|e| format!("Failed to create log directory: {} - {}", log_path.display(), e))?;
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

        let log_entry = if type_name == "Removed" {
                format!("{} -> {}: {}\\{}\n", time_info, type_name, hkey_text, subkey)
        } else {
                format!(
                        "{} -> {}\\{}\\{}\\{} -> {}\n",
                        time_info, hkey_text, subkey, value_name, type_name, value
                )
        };

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

pub fn center() -> (i32, i32) {
        ((app::screen_size().0 / 2.0) as i32, (app::screen_size().1 / 2.0) as i32)
}
