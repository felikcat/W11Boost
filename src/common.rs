use anyhow::Result;
use anyhow::anyhow;
use core::fmt::Write as _;
use std::fs::{self, OpenOptions};
use std::io::Write as _;
use std::os::windows::process::CommandExt as _;
use std::path::PathBuf;
use std::process::Command;
use std::sync::OnceLock;
use winsafe::co::{ERROR, MB, REG};
use winsafe::prelude::*;
use winsafe::{
        self as w, GetLocalTime, HKEY, HWND, RegistryValue,
        co::{self, KNOWNFOLDERID},
};
use windows::Win32::System::Registry::{RegSetValueExW, REG_SZ};

static CACHE: Cache = Cache::new();

pub const CREATE_NO_WINDOW: u32 = 0x0800_0000;

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
        let documents_dir = get_windows_path(&KNOWNFOLDERID::Documents).unwrap_or_else(|e| {
                let msg = format!("Failed to get the Documents path, W11Boost will close.\nError: {e}");
                let _ = HWND::NULL.MessageBox(&msg, "W11Boost Error", MB::OK | MB::ICONERROR);
                panic!("Windows path failure")
        });

        let mut log_path = PathBuf::from(documents_dir);
        log_path.push("W11Boost - do not remove");

        if !log_path.exists() {
                let _ = fs::create_dir_all(log_path.as_path())
                        .map_err(|e| panic!("Failed to create log path.\nError: {e}"));
        }

        log_path
}

pub fn get_windows_path(folder_id: &KNOWNFOLDERID) -> Result<String>
{
        let the_path = w::SHGetKnownFolderPath(folder_id, co::KF::DEFAULT, None)?;
        Ok(the_path)
}

pub fn registry_backup(hkey: &HKEY, subkey: &str, value_name: &str, is_removal: bool) -> Result<()>
{
        let hkey_text = get_hkey_text(hkey);
        let log_path = CACHE.lp();
        let backup_file = log_path.join("Registry Backup.log");

        if is_removal {
                return registry_recursive_backup(hkey, subkey, &backup_file, hkey_text);
        }

        if backup_file.exists() {
                let content = fs::read_to_string(&backup_file)?;
                let key_identifier = format!("{hkey_text}|{subkey}|{value_name}");

                // Skip already backed up registry keys.
                if content.lines().any(|line| line.starts_with(&key_identifier)) {
                        return Ok(());
                }
        }

        let backup_line = match hkey.RegOpenKeyEx(Some(subkey), co::REG_OPTION::NoValue, co::KEY::READ) {
                Ok(key) => {
                        // Find the specific value and get its type.
                        // Collect iterator results first to release the iterator before reading values.
                        let values: Vec<_> = key.RegEnumValue()?.collect();
                        let mut found_value = None;
                        for value_result in values {
                                let (name, reg_type) = value_result?;
                                if name == value_name {
                                        found_value = Some(reg_type);
                                        break;
                                }
                        }

                        let result = match found_value {
                                Some(REG::DWORD) => match key.RegGetValue(None, Some(value_name), co::RRF::RT_DWORD) {
                                        Ok(RegistryValue::Dword(val)) => {
                                                format!("{hkey_text}|{subkey}|{value_name}|DWORD:{val}\n")
                                        }
                                        _ => anyhow!("Failed to read DWORD value").to_string(),
                                },
                                Some(REG::SZ) => match key.RegGetValue(None, Some(value_name), co::RRF::RT_REG_SZ) {
                                        Ok(RegistryValue::Sz(val)) => {
                                                format!("{hkey_text}|{subkey}|{value_name}|SZ:{val}\n")
                                        }
                                        _ => anyhow!("Failed to read SZ value").to_string(),
                                },
                                None => {
                                        // Key exists, its value doesn't.
                                        format!("{hkey_text}|{subkey}|{value_name}|KEY_CREATED_BY_APP\n")
                                }
                                _ => anyhow!("Unsupported registry type").to_string(),
                        };
                        drop(key); // Explicitly close handle
                        result
                }
                Err(e) if e == ERROR::FILE_NOT_FOUND => {
                        // Key doesn't exist, mark for subkey removal during restoration.
                        format!("{hkey_text}|{subkey}|SUBKEY|KEY_CREATED_BY_APP\n")
                }
                Err(e) => anyhow!("Failed to open registry key: {e}\n").to_string(),
        };

        let mut content = if backup_file.exists() {
                fs::read_to_string(&backup_file)?
        } else {
                String::new()
        };

        if !content.contains(&backup_line) {
                content.push_str(&backup_line);
                fs::write(&backup_file, content)?;
        }

        Ok(())
}

pub fn registry_recursive_backup(hkey: &HKEY, subkey: &str, backup_file: &PathBuf, hkey_text: &str) -> Result<()>
{
        let key = match hkey.RegOpenKeyEx(Some(subkey), co::REG_OPTION::NoValue, co::KEY::READ) {
                Ok(key) => key,
                Err(e) if e == ERROR::FILE_NOT_FOUND => {
                        let backup_line = format!("{hkey_text}|{subkey}|SUBKEY|NOT_FOUND\n");
                        let mut content = if backup_file.exists() {
                                fs::read_to_string(backup_file)?
                        } else {
                                String::new()
                        };
                        content.push_str(&backup_line);
                        fs::write(backup_file, content)?;
                        return Ok(());
                }
                Err(e) => {
                        return Err(anyhow!(
                                "Failed to open subkey for backup: {hkey_text}\\{subkey}\nError: {e}"
                        ));
                }
        };

        let mut content = if backup_file.exists() {
                fs::read_to_string(backup_file)?
        } else {
                String::new()
        };

        writeln!(content, "{hkey_text}|{subkey}|SUBKEY|BEGIN_SUBKEY\n")?;

        // Collect iterator results first to release the iterator before reading values.
        let values: Vec<_> = key.RegEnumValue()?.collect();
        for value_result in values {
                let (value_name, reg_type) = value_result?;
                let value_line = match reg_type {
                        REG::DWORD => match key.RegGetValue(None, Some(&value_name), co::RRF::RT_DWORD) {
                                Ok(RegistryValue::Dword(val)) => {
                                        format!("{hkey_text}|{subkey}|{value_name}|DWORD:{val}\n")
                                }
                                _ => {
                                        return Err(anyhow!(
                                                "{hkey_text}|{subkey}|{value_name}|DWORD:ERROR_READING_VALUE\n"
                                        ));
                                }
                        },
                        REG::SZ => match key.RegGetValue(None, Some(&value_name), co::RRF::RT_REG_SZ) {
                                Ok(RegistryValue::Sz(val)) => {
                                        format!("{hkey_text}|{subkey}|{value_name}|SZ:{val}\n")
                                }
                                _ => return Err(anyhow!("{hkey_text}|{subkey}|{value_name}|SZ:ERROR_READING_VALUE\n")),
                        },
                        _ => return Err(anyhow!("{hkey_text}|{subkey}|{value_name}|UNKNOWN_TYPE\n")),
                };
                content.push_str(&value_line);
        }

        drop(key); // Explicitly close handle
        writeln!(content, "{hkey_text}|{subkey}|SUBKEY|END_SUBKEY\n")?;

        fs::write(backup_file, content)?;
        Ok(())
}

pub fn restore_from_backup() -> Result<()>
{
        let log_path = CACHE.lp();
        let backup_file = log_path.join("Registry Backup.log");

        if !backup_file.exists() {
                return Err(anyhow!("No backup file found!"));
        }

        let content = fs::read_to_string(&backup_file)?;
        let mut restored = 0;
        let mut skipped = 0;

        for line in content.lines() {
                if line.trim().is_empty() {
                        continue;
                }

                match restore_single_line(line) {
                        Ok(true) => restored += 1,
                        Ok(false) => skipped += 1,
                        Err(e) => return Err(anyhow!("Failed to restore: {line}\nError: {e}")),
                }
        }

        println!("Uninstall complete!\nRestored {restored} registry keys, skipped {skipped} registry keys.");
        Ok(())
}

// Expected behavior:
// - Delete subkey if created by W11Boost.
// - Set the values back if the key originally existed before W11Boost.
// - Create the subkey (if BEGIN_SUBKEY) so that keys inside that subkey can also be created. Can think of it like creating a directory.
fn restore_single_line(line: &str) -> Result<bool>
{
        let parts: Vec<&str> = line.split('|').collect();

        let hkey = match parts[0] {
                "HKEY_LOCAL_MACHINE" => HKEY::LOCAL_MACHINE,
                "HKEY_CURRENT_USER" => HKEY::CURRENT_USER,
                _ => return Err(anyhow!("Unknown HKEY: {}", parts[0])),
        };

        let subkey = parts[1];
        let value_name = parts[2];
        let value_info = parts[3];

        match value_info {
                "NOT_FOUND" | "END_SUBKEY" => return Ok(false),
                "KEY_CREATED_BY_APP" => {
                        match hkey.RegDeleteTree(Some(subkey)) {
                                Ok(()) => return Ok(true),
                                Err(_) => return Ok(false),
                        };
                }
                "BEGIN_SUBKEY" => {
                        match hkey.RegCreateKeyEx(subkey, None, co::REG_OPTION::NON_VOLATILE, co::KEY::WRITE, None) {
                                Ok(_) => return Ok(true),
                                Err(_) => return Ok(false),
                        }
                }
                _ => {}
        }

        // strip_prefix instead of strip_suffix since "DWORD:" is not the end of the string.
        if let Some(dword_str) = value_info.strip_prefix("DWORD:") {
                let value: u32 = dword_str.parse()?;
                set_dword(&hkey, subkey, value_name, value)?;
                Ok(true)
        } else if let Some(string_str) = value_info.strip_prefix("SZ:") {
                set_string(&hkey, subkey, value_name, string_str)?;
                Ok(true)
        } else {
                Err(anyhow!("Unknown registry value format: {value_info}"))
        }
}

pub fn set_dword(hkey: &HKEY, subkey: &str, value_name: &str, value: u32) -> Result<()>
{
        let hkey_text = get_hkey_text(hkey);

        registry_backup(hkey, subkey, value_name, false)?;

        let (key, _) = hkey
                .RegCreateKeyEx(subkey, None, co::REG_OPTION::NON_VOLATILE, co::KEY::WRITE, None)
                .map_err(|source| {
                        anyhow!(
                                "Failed to open DWORD registry key: {}\\{}\\{}\nError: {}",
                                hkey_text,
                                subkey,
                                value_name,
                                source
                        )
                })?;

        let result = key.RegSetValueEx(Some(value_name), RegistryValue::Dword(value));
        drop(key); // Explicitly close handle before continuing

        result.map_err(|source| {
                anyhow!(
                        "Failed to set DWORD registry value: {}\\{}\\{} = {}\nError: {}",
                        hkey_text,
                        subkey,
                        value_name,
                        value,
                        source
                )
        })?;

        log_registry(hkey, subkey, value_name, &value.to_string(), "DWORD").map_err(|e| {
                anyhow!(
                        "Failed to log DWORD change for key: {}\\{}\\{} -> {}\nError: {}",
                        hkey_text,
                        subkey,
                        value_name,
                        value,
                        e
                )
        })?;

        Ok(())
}

pub fn set_string(hkey: &HKEY, subkey: &str, value_name: &str, value: &str) -> Result<()>
{
        let hkey_text = get_hkey_text(hkey);

        registry_backup(hkey, subkey, value_name, false)?;

        let (key, _) = hkey
                .RegCreateKeyEx(subkey, None, co::REG_OPTION::NON_VOLATILE, co::KEY::WRITE, None)
                .map_err(|source| {
                        anyhow!(
                                "Failed to open Sz registry key: {}\\{}\\{}\nError: {}",
                                hkey_text,
                                subkey,
                                value_name,
                                source
                        )
                })?;

        // Use raw Windows API for all strings to handle empty strings correctly.
        // winsafe's RegistryValue::Sz doesn't handle empty strings (ERROR 998).
        let wide_value_name: Vec<u16> = value_name.encode_utf16().chain(std::iter::once(0)).collect();
        let wide_value: Vec<u16> = value.encode_utf16().chain(std::iter::once(0)).collect();
        let byte_len = wide_value.len() * 2;

        let result = unsafe {
                RegSetValueExW(
                        windows::Win32::System::Registry::HKEY(key.ptr() as *mut _),
                        windows::core::PCWSTR(wide_value_name.as_ptr()),
                        None,
                        REG_SZ,
                        Some(std::slice::from_raw_parts(wide_value.as_ptr() as *const u8, byte_len)),
                )
        };

        drop(key);

        if result.is_err() {
                return Err(anyhow!(
                        "Failed to set Sz registry value: {}\\{}\\{} = {}\nError: {}",
                        hkey_text,
                        subkey,
                        value_name,
                        value,
                        result.0
                ));
        }

        log_registry(hkey, subkey, value_name, value, "String").map_err(|e| {
                anyhow!(
                        "Failed to log Sz change for key: {}\\{}\\{} -> {}\nError: {}",
                        hkey_text,
                        subkey,
                        value_name,
                        value,
                        e
                )
        })?;

        Ok(())
}

pub fn remove_subkey(hkey: &HKEY, subkey: &str) -> Result<()>
{
        let hkey_text = get_hkey_text(hkey);

        registry_backup(hkey, subkey, "", true)?;

        match hkey.RegDeleteTree(Some(subkey)) {
                Ok(()) => Ok(()),
                Err(e) if e == ERROR::FILE_NOT_FOUND => Ok(()),
                Err(e) => Err(anyhow!(
                        "Failed to delete subkey: {}\\{}\nError: {}",
                        hkey_text,
                        subkey,
                        e
                )),
        }?;

        log_registry(hkey, subkey, "->", "", "Removed")
                .map_err(|e| anyhow!("Failed to log removal of key: {}\\{}\nError: {}", hkey_text, subkey, e))?;
        Ok(())
}

// Deletes a single value from a registry key without affecting other values or the key itself.
// Returns Ok(()) if value was deleted or didn't exist.
pub fn delete_value(hkey: &HKEY, subkey: &str, value_name: &str) -> Result<()>
{
        let hkey_text = get_hkey_text(hkey);

        let key = match hkey.RegOpenKeyEx(Some(subkey), co::REG_OPTION::NoValue, co::KEY::SET_VALUE) {
                Ok(k) => k,
                Err(e) if e == ERROR::FILE_NOT_FOUND => return Ok(()), // Key doesn't exist = value doesn't exist
                Err(e) => {
                        return Err(anyhow!(
                                "Failed to open key for value deletion: {}\\{}\nError: {}",
                                hkey_text,
                                subkey,
                                e
                        ))
                }
        };

        let result = key.RegDeleteValue(Some(value_name));
        drop(key); // Explicitly close handle before returning

        match result {
                Ok(()) => Ok(()),
                Err(e) if e == ERROR::FILE_NOT_FOUND => Ok(()), // Value doesn't exist = success
                Err(e) => Err(anyhow!(
                        "Failed to delete value: {}\\{}\\{}\nError: {}",
                        hkey_text,
                        subkey,
                        value_name,
                        e
                )),
        }
}

pub fn check_dword(hkey: &HKEY, subkey: &str, value_name: &str, expected_value: u32) -> Result<bool>
{
        let hkey_text = get_hkey_text(hkey);

        let reg_value = match hkey.RegGetValue(Some(subkey), Some(value_name), co::RRF::RT_DWORD) {
                Ok(value) => value,
                Err(e) if e == w::co::ERROR::FILE_NOT_FOUND => return Ok(false),
                Err(e) => {
                        return Err(anyhow!(
                                "Failed to open key for DWORD check: {}\\{}\\{}\nError: {}",
                                hkey_text,
                                subkey,
                                value_name,
                                e
                        ));
                }
        };

        match reg_value {
                RegistryValue::Dword(value) => Ok(value == expected_value),
                _ => Err(anyhow!(
                        "Expected DWORD value but found different type for: {}\\{}\\{}",
                        hkey_text,
                        subkey,
                        value_name
                )),
        }
}

fn get_hkey_text(hkey: &HKEY) -> &str
{
        if *hkey == HKEY::LOCAL_MACHINE {
                "HKEY_LOCAL_MACHINE"
        } else if *hkey == HKEY::CURRENT_USER {
                "HKEY_CURRENT_USER"
        } else {
                "UNKNOWN_HKEY"
        }
}

fn log_registry(hkey: &HKEY, subkey: &str, value_name: &str, value: &str, type_name: &str) -> Result<()>
{
        let hkey_text = get_hkey_text(hkey);
        let log_path = CACHE.lp();

        if !log_path.exists() {
                fs::create_dir_all(log_path.as_path())
                        .map_err(|e| anyhow!("Failed to create log directory: {}\nError: {e}", log_path.display()))?;
        }

        let now = GetLocalTime();
        let time_info = format!(
                "{:02}/{:02}/{} {:02}:{:02}:{:02}",
                now.wDay, now.wMonth, now.wYear, now.wHour, now.wMinute, now.wSecond
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

pub fn run_system_command(program: &str, args: &[&str]) -> Result<()>
{
        Command::new(program)
                .args(args)
                .creation_flags(CREATE_NO_WINDOW)
                .output()
                .map_err(|e| anyhow!("Failed to execute {program}.\nError: {e}"))?;

        Ok(())
}
