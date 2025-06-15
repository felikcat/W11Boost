use chrono::{Datelike, Local, Timelike};
use fltk::{app, dialog};
use thiserror::Error;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::OnceLock;
use winsafe::co::ERROR;
use winsafe::{
        self as w, HKEY, RegistryValue,
        co::{self, KNOWNFOLDERID},
        prelude::*,
};

static CACHE: Cache = Cache::new();

pub const CREATE_NO_WINDOW: u32 = 0x08000000;

struct Cache {
        cache: OnceLock<PathBuf>,
}

impl Cache {
        const fn new() -> Self {
                Self { cache: OnceLock::new() }
        }

        fn lp(&self) -> &PathBuf {
                self.cache.get_or_init(log_path)
        }
}

fn log_path() -> PathBuf {
        let desktop_dir = get_windows_path(&KNOWNFOLDERID::Desktop).unwrap_or_else(|e| {
                dialog::alert(
                        center().0,
                        center().1,
                        &format!(
                                "Failed to get the Desktop Windows path, W11Boost will close.\nError: {}",
                                e
                        ),
                );
                // We want the program to force close in this case.
                panic!("Windows path failure")
        });

        let mut log_path = PathBuf::from(desktop_dir);
        log_path.push(r"W11Boost Logs");

        log_path
}

pub fn get_windows_path(folder_id: &KNOWNFOLDERID) -> Result<String, Box<dyn Error>> {
        let the_path = w::SHGetKnownFolderPath(folder_id, co::KF::DEFAULT, None)?;
        Ok(the_path)
}

#[derive(Debug, Error)]
pub enum RegistryError {
        #[error("Failed to open registry key: {hkey}\\{subkey}\\{value_name}\nError: {source}")]
        KeyOpenFailed {
                hkey: String,
                subkey: String,
                value_name: String,
                #[source]
                source: ERROR,
        },
        #[error("Failed to set registry value: {hkey}\\{subkey}\\{value_name} = {value}\nError: {source}")]
        ValueSetFailed {
                hkey: String,
                subkey: String,
                value_name: String,
                value: String,
                #[source]
                source: ERROR,
        },
        #[error("Failed to delete subkey: {hkey}\\{subkey}\nError: {source}")]
        SubkeyRemoveFailed {
                hkey: String,
                subkey: String,
                #[source]
                source: ERROR,
        },
}

pub fn set_dword(hkey: &HKEY, subkey: &str, value_name: &str, value: u32) -> Result<(), Box<dyn Error>> {
        let o_subkey = subkey;
        let hkey_text = get_hkey_text(hkey)?;

        let (subkey, _) = hkey
                .RegCreateKeyEx(subkey, None, co::REG_OPTION::NON_VOLATILE, co::KEY::WRITE, None)
                .map_err(|source| RegistryError::KeyOpenFailed {
                        hkey: hkey_text.to_string(),
                        subkey: subkey.to_string(),
                        value_name: value_name.to_string(),
                        source,
                })?;

        subkey.RegSetValueEx(Some(value_name), RegistryValue::Dword(value))
                .map_err(|source| RegistryError::ValueSetFailed {
                        hkey: hkey_text.to_string(),
                        subkey: subkey.to_string(),
                        value_name: value_name.to_string(),
                        value: value.to_string(),
                        source,
                })?;

        log_registry(hkey, o_subkey, value_name, &value.to_string(), "DWORD").map_err(|e| {
                format!(
                        "Failed to log DWORD change for key: {}\\{}\\{} -> {}\nError: {}",
                        hkey_text, o_subkey, value_name, value, e
                )
        })?;

        Ok(())
}

pub fn set_string(hkey: &HKEY, subkey: &str, value_name: &str, value: &str) -> Result<(), Box<dyn Error>> {
        let o_subkey = subkey;
        let hkey_text = get_hkey_text(hkey)?;

        let (subkey, _) = hkey
                .RegCreateKeyEx(subkey, None, co::REG_OPTION::NON_VOLATILE, co::KEY::WRITE, None)
                .map_err(|source| RegistryError::KeyOpenFailed {
                        hkey: hkey_text.to_string(),
                        subkey: subkey.to_string(),
                        value_name: value_name.to_string(),
                        source,
                })?;

        let value = value.to_string();
        subkey.RegSetValueEx(Some(value_name), RegistryValue::Sz(value.clone()))
                .map_err(|source| RegistryError::ValueSetFailed {
                        hkey: hkey_text.to_string(),
                        subkey: subkey.to_string(),
                        value_name: value_name.to_string(),
                        value: value.to_string(),
                        source,
                })?;

        log_registry(hkey, o_subkey, value_name, &value, "String").map_err(|e| {
                format!(
                        "Failed to log Sz change for key: {}\\{}\\{} -> {}\nError: {}",
                        hkey_text, o_subkey, value_name, value, e
                )
        })?;

        Ok(())
}

pub fn remove_subkey(hkey: &HKEY, subkey: &str) -> Result<(), Box<dyn Error>> {
        let o_subkey = subkey;
        let hkey_text = get_hkey_text(hkey)?;

        match hkey.RegDeleteTree(Some(subkey)) {
                Ok(_) => Ok(()),
                Err(e) if e == ERROR::FILE_NOT_FOUND => Ok(()),
                Err(source) => Err(RegistryError::SubkeyRemoveFailed {
                        hkey: hkey_text.to_string(),
                        subkey: o_subkey.to_string(),
                        source,
                })
                .into(),
        }?;

        log_registry(hkey, o_subkey, "->", "", "Removed").map_err(|e| {
                format!(
                        "Failed to log removal of key: {}\\{}\nError: {}",
                        hkey_text, o_subkey, e
                )
        })?;
        Ok(())
}

pub fn check_dword(hkey: &HKEY, subkey: &str, value_name: &str, expected_value: u32) -> Result<bool, Box<dyn Error>> {
        let o_subkey = subkey;
        let hkey_text = get_hkey_text(hkey)?;

        let subkey = match hkey.RegGetValue(Some(subkey), Some(value_name), co::RRF::RT_DWORD) {
                Ok(value) => value,
                Err(e) if e == w::co::ERROR::FILE_NOT_FOUND => return Ok(false),
                Err(e) => {
                        return Err(format!(
                                "Failed to open key for DWORD check: {}\\{}\\{}\nError: {}",
                                hkey_text, o_subkey, value_name, e
                        )
                        .into());
                }
        };

        if let RegistryValue::Dword(value) = subkey {
                if value != expected_value { Ok(false) } else { Ok(true) }
        } else {
                Err(format!("Expected DWORD value for: {}\\{}\\{}", hkey_text, o_subkey, value_name).into())
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
        let log_path = CACHE.lp();

        if !log_path.exists() {
                fs::create_dir_all(log_path.as_path())
                        .map_err(|e| format!("Failed to create log directory: {}\nError: {}", log_path.display(), e))?;
        }

        let now = Local::now();
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
                        "{} -> {}: {}\\{}\\{} -> {}\n",
                        time_info, type_name, hkey_text, subkey, value_name, value
                )
        };

        let log_file = log_path.join("Registry.log");

        let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(log_file.as_path())
                .map_err(|e| {
                        format!(
                                "Failed to open/create log file: {}{}\nError: {}",
                                log_path.display(),
                                log_file.display(),
                                e
                        )
                })?;

        file.write_all(log_entry.as_bytes()).map_err(|e| {
                format!(
                        "Failed to write to log file: {}{}\nError: {}",
                        log_path.display(),
                        log_file.display(),
                        e
                )
        })?;

        Ok(())
}

pub fn center() -> (i32, i32) {
        ((app::screen_size().0 / 2.0) as i32, (app::screen_size().1 / 2.0) as i32)
}
