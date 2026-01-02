use anyhow::Result;
use anyhow::anyhow;
use chrono::{Datelike as _, Local, Timelike as _};
use fltk::{app, dialog};
use std::fs::{self, OpenOptions};
use std::io::Write as _;
use std::os::windows::process::CommandExt as _;
use std::path::PathBuf;
use std::process::Command;
use std::sync::OnceLock;
use winsafe::co::{ERROR, REG};
use winsafe::{
        self as w, HKEY, RegistryValue,
        co::{self, KNOWNFOLDERID},
};
use core::fmt::Write as _;

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
                dialog::alert(
                        center().0,
                        center().1,
                        &format!("Failed to get the Desktop Windows path, W11Boost will close.\nError: {e}"),
                );
                // We want the program to force close in this case.
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

        let backup_line = match hkey.RegOpenKeyEx(Some(subkey), co::REG_OPTION::NON_VOLATILE, co::KEY::READ) {
                Ok(key) => {
                        // Find the specific value and get its type.
                        let mut found_value = None;
                        for value_result in key.RegEnumValue()? {
                                let (name, reg_type) = value_result?;
                                if name == value_name {
                                        found_value = Some(reg_type);
                                        break;
                                }
                        }

                        match found_value {
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
                        }
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

        for value_result in key.RegEnumValue()? {
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
        let o_subkey = subkey;
        let hkey_text = get_hkey_text(hkey);

        registry_backup(hkey, subkey, value_name, false)?;

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

pub fn set_string(hkey: &HKEY, subkey: &str, value_name: &str, value: &str) -> Result<()>
{
        let o_subkey = subkey;
        let hkey_text = get_hkey_text(hkey);

        registry_backup(hkey, subkey, value_name, false)?;

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

        let value = value.to_owned();
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

pub fn remove_subkey(hkey: &HKEY, subkey: &str) -> Result<()>
{
        let o_subkey = subkey;
        let hkey_text = get_hkey_text(hkey);

        registry_backup(hkey, subkey, "", true)?;

        match hkey.RegDeleteTree(Some(subkey)) {
                Ok(()) => Ok(()),
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

pub fn check_dword(hkey: &HKEY, subkey: &str, value_name: &str, expected_value: u32) -> Result<bool>
{
        let o_subkey = subkey;
        let hkey_text = get_hkey_text(hkey);

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

        match subkey {
        	RegistryValue::Dword(value) => {
        		if value == expected_value {
        			Ok(true)
        		} else {
        			Ok(false)
        		}
        	}
        	_ => Err(anyhow!(
        		"Expected DWORD value but found different type for: {}\\{}\\{}",
        		hkey_text,
        		o_subkey,
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

#[must_use]
#[expect(clippy::cast_possible_truncation)]
pub fn center() -> (i32, i32)
{
        ((app::screen_size().0 / 2.0) as i32, (app::screen_size().1 / 2.0) as i32)
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

// Barrett reduction constants for different divisors.
// For 32-bit integers: m = floor((2^k) / d) where k=35
pub struct BarrettConstants
{
        m: u64,
        k: u32,
}

pub const BARRETT_DIV_12: BarrettConstants = BarrettConstants {
        m: 2_863_311_530, // floor(2^35 / 12) = floor(34359738368 / 12) = 2863311530
        k: 35,
};

pub const BARRETT_DIV_100: BarrettConstants = BarrettConstants {
        m: 343_597_383, // floor(2^35 / 100) = floor(34359738368 / 100) = 343597383
        k: 35,
};

/// Generic Barrett reduction for constant-time division.
/// Replaces "n / divisor" with multiplication and bit shifts for better security & performance.
#[inline] #[must_use]
pub fn barrett_div(n: i32, constants: &BarrettConstants) -> i32
{
        // Barrett reduction: q = floor((n * m) >> k)
        // where m and k are precomputed constants.
        let n_u64 = n as u64;
        let product = n_u64 * constants.m;
        i32::try_from(product >> constants.k).unwrap_or_else(|_| {
                // Fallback to direct casting if try_from fails (it shouldn't).
                (product >> constants.k) as i32
        })
}

