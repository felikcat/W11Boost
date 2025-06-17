use anyhow::anyhow;
use chrono::{Datelike, Local, Timelike};
use fltk::{app, dialog};
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

struct Cache
{
        cache: OnceLock<PathBuf>,
}

impl Cache
{
        const fn new() -> Self
        {
                Self { cache: OnceLock::new() }
        }

        fn lp(&self) -> &PathBuf
        {
                self.cache.get_or_init(log_path)
        }
}

fn log_path() -> PathBuf
{
        let desktop_dir = get_windows_path(&KNOWNFOLDERID::Desktop).unwrap_or_else(|e| {
                dialog::alert(
                        center().0,
                        center().1,
                        &format!("Failed to get the Desktop Windows path, W11Boost will close.\nError: {e}"),
                );
                // We want the program to force close in this case.
                panic!("Windows path failure")
        });

        let mut log_path = PathBuf::from(desktop_dir);
        log_path.push(r"W11Boost Logs");

        log_path
}

pub fn get_windows_path(folder_id: &KNOWNFOLDERID) -> anyhow::Result<String>
{
        let the_path = w::SHGetKnownFolderPath(folder_id, co::KF::DEFAULT, None)?;
        Ok(the_path)
}

pub fn set_dword(hkey: &HKEY, subkey: &str, value_name: &str, value: u32) -> anyhow::Result<()>
{
        let o_subkey = subkey;
        let hkey_text = get_hkey_text(hkey)?;

        let (subkey, _) = hkey
                .RegCreateKeyEx(subkey, None, co::REG_OPTION::NON_VOLATILE, co::KEY::WRITE, None)
                .map_err(|source| {
                        anyhow!(
                                "Failed to open DWORD registry key: {}\\{}\\{}\nError: {}",
                                hkey_text,
                                o_subkey,
                                value_name,
                                source
                        )
                })?;

        subkey.RegSetValueEx(Some(value_name), RegistryValue::Dword(value))
                .map_err(|source| {
                        anyhow!(
                                "Failed to set DWORD registry value: {}\\{}\\{} = {}\nError: {}",
                                hkey_text,
                                o_subkey,
                                value_name,
                                value,
                                source
                        )
                })?;

        log_registry(hkey, o_subkey, value_name, &value.to_string(), "DWORD").map_err(|e| {
                anyhow!(
                        "Failed to log DWORD change for key: {}\\{}\\{} -> {}\nError: {}",
                        hkey_text,
                        o_subkey,
                        value_name,
                        value,
                        e
                )
        })?;

        Ok(())
}

pub fn set_string(hkey: &HKEY, subkey: &str, value_name: &str, value: &str) -> anyhow::Result<()>
{
        let o_subkey = subkey;
        let hkey_text = get_hkey_text(hkey)?;

        let (subkey, _) = hkey
                .RegCreateKeyEx(subkey, None, co::REG_OPTION::NON_VOLATILE, co::KEY::WRITE, None)
                .map_err(|source| {
                        anyhow!(
                                "Failed to open Sz registry key: {}\\{}\\{}\nError: {}",
                                hkey_text,
                                o_subkey,
                                value_name,
                                source
                        )
                })?;

        let value = value.to_string();
        subkey.RegSetValueEx(Some(value_name), RegistryValue::Sz(value.clone()))
                .map_err(|source| {
                        anyhow!(
                                "Failed to set Sz registry value: {}\\{}\\{} = {}\nError: {}",
                                hkey_text,
                                o_subkey,
                                value_name,
                                value,
                                source
                        )
                })?;

        log_registry(hkey, o_subkey, value_name, &value, "String").map_err(|e| {
                anyhow!(
                        "Failed to log Sz change for key: {}\\{}\\{} -> {}\nError: {}",
                        hkey_text,
                        o_subkey,
                        value_name,
                        value,
                        e
                )
        })?;

        Ok(())
}

pub fn remove_subkey(hkey: &HKEY, subkey: &str) -> anyhow::Result<()>
{
        let o_subkey = subkey;
        let hkey_text = get_hkey_text(hkey)?;

        match hkey.RegDeleteTree(Some(subkey)) {
                Ok(_) => Ok(()),
                Err(e) if e == ERROR::FILE_NOT_FOUND => Ok(()),
                Err(e) => Err(anyhow!(
                        "Failed to delete subkey: {}\\{}\nError: {}",
                        hkey_text,
                        o_subkey,
                        e
                )),
        }?;

        log_registry(hkey, o_subkey, "->", "", "Removed").map_err(|e| {
                anyhow!(
                        "Failed to log removal of key: {}\\{}\nError: {}",
                        hkey_text,
                        o_subkey,
                        e
                )
        })?;
        Ok(())
}

pub fn check_dword(hkey: &HKEY, subkey: &str, value_name: &str, expected_value: u32) -> anyhow::Result<bool>
{
        let o_subkey = subkey;
        let hkey_text = get_hkey_text(hkey)?;

        let subkey = match hkey.RegGetValue(Some(subkey), Some(value_name), co::RRF::RT_DWORD) {
                Ok(value) => value,
                Err(e) if e == w::co::ERROR::FILE_NOT_FOUND => return Ok(false),
                Err(e) => {
                        return Err(anyhow!(
                                "Failed to open key for DWORD check: {}\\{}\\{}\nError: {}",
                                hkey_text,
                                o_subkey,
                                value_name,
                                e
                        ));
                }
        };

        if let RegistryValue::Dword(value) = subkey {
                if value != expected_value { Ok(false) } else { Ok(true) }
        } else {
                Err(anyhow!(
                        "Expected DWORD value for: {}\\{}\\{}",
                        hkey_text,
                        o_subkey,
                        value_name
                ))
        }
}

fn get_hkey_text(hkey: &HKEY) -> anyhow::Result<&str>
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

fn log_registry(hkey: &HKEY, subkey: &str, value_name: &str, value: &str, type_name: &str) -> anyhow::Result<()>
{
        let hkey_text = get_hkey_text(hkey)?;
        let log_path = CACHE.lp();

        if !log_path.exists() {
                fs::create_dir_all(log_path.as_path())
                        .map_err(|e| anyhow!("Failed to create log directory: {}\nError: {e}", log_path.display()))?;
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
                format!("{time_info} -> {type_name}: {hkey_text}\\{subkey}\n")
        } else {
                format!("{time_info} -> {type_name}: {hkey_text}\\{subkey}\\{value_name} -> {value}\n")
        };

        let log_file = log_path.join("Registry.log");

        let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(log_file.as_path())
                .map_err(|e| {
                        anyhow!(
                                "Failed to open/create log file: {}{}\nError: {}",
                                log_path.display(),
                                log_file.display(),
                                e
                        )
                })?;

        file.write_all(log_entry.as_bytes()).map_err(|e| {
                anyhow!(
                        "Failed to write to log file: {}{}\nError: {e}",
                        log_path.display(),
                        log_file.display(),
                )
        })?;

        Ok(())
}

pub fn center() -> (i32, i32)
{
        ((app::screen_size().0 / 2.0) as i32, (app::screen_size().1 / 2.0) as i32)
}
