use crate::trusted_installer::TrustedInstallerGuard;
use anyhow::{Context, Result, anyhow};
use std::fs::{self, OpenOptions};
use std::io::Write as _;
use std::os::windows::process::CommandExt as _;
use std::path::PathBuf;
use std::process::Command;
use std::sync::OnceLock;
use windows::Win32::System::Com::{CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx};

pub fn log_debug(component: &str, msg: &str)
{
        let _ = std::fs::create_dir_all(r"C:\ProgramData\W11Boost");
        let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open(format!(r"C:\ProgramData\W11Boost\{}_debug.log", component))
        else {
                return;
        };

        unsafe {
                let time = windows::Win32::System::SystemInformation::GetLocalTime();
                let _ = writeln!(
                        file,
                        "[{:02}:{:02}:{:02}] {}",
                        time.wHour, time.wMinute, time.wSecond, msg
                );
        }
}
use windows::Win32::System::GroupPolicy::{
        CLSID_GroupPolicyObject, GPO_OPEN_LOAD_REGISTRY, GPO_SECTION_MACHINE, IGroupPolicyObject,
        REGISTRY_EXTENSION_GUID,
};
use windows::Win32::System::Registry::{
        REG_BINARY, REG_DWORD, REG_EXPAND_SZ, REG_SZ, RegCloseKey, RegCreateKeyExW, RegSetValueExW,
};
use windows::core::{GUID, PCWSTR};
use winsafe::co::ERROR;

use winsafe::{
        self as w, GetLocalTime, HKEY, RegistryValue,
        co::{self, KNOWNFOLDERID},
};

use crate::ipc::{RegRoot, ServiceCommand};
use crate::service_client;
use crate::trusted_installer;

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

        fn lp(&self) -> Result<&PathBuf>
        {
                if let Some(path) = self.cache.get() {
                        return Ok(path);
                }
                let path = log_path()?;
                let _ = self.cache.set(path);
                Ok(self.cache.get().expect("Cache should be set"))
        }
}

fn log_path() -> Result<PathBuf>
{
        // Use ProgramData (Common AppData) so that both the user-mode GUI and the SYSTEM-mode service
        // can write to the same log location without permission issues (SYSTEM often lacks a Documents folder).
        let program_data_dir = get_windows_path(&KNOWNFOLDERID::ProgramData)
                .context("Failed to get ProgramData folder for logging")?;

        let mut log_path = PathBuf::from(program_data_dir);
        log_path.push("W11Boost");
        log_path.push("Logs");

        if !log_path.exists() {
                fs::create_dir_all(&log_path).context(format!("Failed to create log directory at {:?}", log_path))?;
        }

        Ok(log_path)
}

pub fn get_windows_path(folder_id: &KNOWNFOLDERID) -> Result<String>
{
        w::SHGetKnownFolderPath(folder_id, co::KF::DEFAULT, None)
                .map_err(|e| anyhow!("WinSafe error getting known folder: {}", e))
}

pub fn set_dword(hkey: &HKEY, subkey: &str, value_name: &str, value: u32) -> Result<()>
{
        let hkey_text = get_hkey_text(hkey);
        let is_hklm = is_hklm(hkey);

        // If HKLM and we are NOT TrustedInstaller, delegate to Service
        if is_hklm && !trusted_installer::is_trusted_installer() {
                let root = if *hkey == HKEY::LOCAL_MACHINE {
                        RegRoot::HKLM
                } else {
                        return Err(anyhow!("Unsupported root for service delegation"));
                };
                let cmd = ServiceCommand::WriteRegDword {
                        root,
                        subkey: subkey.to_string(),
                        value: value_name.to_string(),
                        data: value,
                };
                match service_client::send_command(cmd) {
                        Ok(crate::ipc::ServiceResponse::Success) => return Ok(()),
                        Ok(crate::ipc::ServiceResponse::Error(msg)) => return Err(anyhow!("Service error: {}", msg)),
                        Err(e) => return Err(anyhow!("Failed to communicate with W11BoostSvc: {}", e)),
                }
        }

        let use_ti = is_hklm; // Legacy var name for local logic

        let set_result = {
                let _ti_guard = if use_ti {
                        Some(TrustedInstallerGuard::new()?)
                } else {
                        None
                };

                // ... rest of local logic
                let result = hkey.RegCreateKeyEx(subkey, None, co::REG_OPTION::NON_VOLATILE, co::KEY::WRITE, None);

                let (key, _) = match result {
                        Ok(k) => k,
                        Err(source) => {
                                return Err(anyhow!(
                                        "Failed to open DWORD registry key: {}\\{}\\{}\nError: {}",
                                        hkey_text,
                                        subkey,
                                        value_name,
                                        source
                                ));
                        }
                };

                key.RegSetValueEx(Some(value_name), RegistryValue::Dword(value))
        };

        set_result.map_err(|source| {
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
        let is_hklm = is_hklm(hkey);

        if is_hklm && !trusted_installer::is_trusted_installer() {
                let root = if *hkey == HKEY::LOCAL_MACHINE {
                        RegRoot::HKLM
                } else {
                        return Err(anyhow!("Unsupported root for service delegation"));
                };
                let cmd = ServiceCommand::WriteRegString {
                        root,
                        subkey: subkey.to_string(),
                        value: value_name.to_string(),
                        data: value.to_string(),
                };
                match service_client::send_command(cmd) {
                        Ok(crate::ipc::ServiceResponse::Success) => return Ok(()),
                        Ok(crate::ipc::ServiceResponse::Error(msg)) => return Err(anyhow!("Service error: {}", msg)),
                        Err(e) => return Err(anyhow!("Failed to communicate with W11BoostSvc: {}", e)),
                }
        }

        let use_ti = is_hklm;

        let result = {
                let _ti_guard = if use_ti {
                        Some(TrustedInstallerGuard::new()?)
                } else {
                        None
                };

                let create_result =
                        hkey.RegCreateKeyEx(subkey, None, co::REG_OPTION::NON_VOLATILE, co::KEY::WRITE, None);

                let (key, _) = match create_result {
                        Ok(k) => k,
                        Err(source) => {
                                return Err(anyhow!(
                                        "Failed to open Sz registry key: {}\\{}\\{}\nError: {}",
                                        hkey_text,
                                        subkey,
                                        value_name,
                                        source
                                ));
                        }
                };

                // Use raw Windows API for all strings to handle empty strings correctly.
                // winsafe's RegistryValue::Sz doesn't handle empty strings (ERROR 998).
                let wide_value_name: Vec<u16> = value_name.encode_utf16().chain(std::iter::once(0)).collect();
                let wide_value: Vec<u16> = value.encode_utf16().chain(std::iter::once(0)).collect();
                let byte_len = wide_value.len() * 2;

                unsafe {
                        RegSetValueExW(
                                windows::Win32::System::Registry::HKEY(key.ptr() as *mut _),
                                windows::core::PCWSTR(wide_value_name.as_ptr()),
                                None,
                                REG_SZ,
                                Some(std::slice::from_raw_parts(wide_value.as_ptr() as *const u8, byte_len)),
                        )
                }
        };

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

pub fn set_expand_sz(hkey: &HKEY, subkey: &str, value_name: &str, value: &str) -> Result<()>
{
        let hkey_text = get_hkey_text(hkey);
        let use_ti = is_hklm(hkey);

        let result = {
                let _ti_guard = if use_ti {
                        Some(TrustedInstallerGuard::new()?)
                } else {
                        None
                };

                let create_result =
                        hkey.RegCreateKeyEx(subkey, None, co::REG_OPTION::NON_VOLATILE, co::KEY::WRITE, None);

                let (key, _) = match create_result {
                        Ok(k) => k,
                        Err(source) => {
                                return Err(anyhow!(
                                        "Failed to open ExpandSz registry key: {}\\{}\\{}\nError: {}",
                                        hkey_text,
                                        subkey,
                                        value_name,
                                        source
                                ));
                        }
                };

                // Use raw Windows API for all strings to handle empty strings correctly.
                let wide_value_name: Vec<u16> = value_name.encode_utf16().chain(std::iter::once(0)).collect();
                let wide_value: Vec<u16> = value.encode_utf16().chain(std::iter::once(0)).collect();
                let byte_len = wide_value.len() * 2;

                unsafe {
                        RegSetValueExW(
                                windows::Win32::System::Registry::HKEY(key.ptr() as *mut _),
                                windows::core::PCWSTR(wide_value_name.as_ptr()),
                                None,
                                REG_EXPAND_SZ,
                                Some(std::slice::from_raw_parts(wide_value.as_ptr() as *const u8, byte_len)),
                        )
                }
        };

        if result.is_err() {
                return Err(anyhow!(
                        "Failed to set ExpandSz registry value: {}\\{}\\{} = {}\nError: {}",
                        hkey_text,
                        subkey,
                        value_name,
                        value,
                        result.0
                ));
        }

        log_registry(hkey, subkey, value_name, value, "ExpandString").map_err(|e| {
                anyhow!(
                        "Failed to log ExpandSz change for key: {}\\{}\\{} -> {}\nError: {}",
                        hkey_text,
                        subkey,
                        value_name,
                        value,
                        e
                )
        })?;

        Ok(())
}

pub fn set_binary(hkey: &HKEY, subkey: &str, value_name: &str, value: &[u8]) -> Result<()>
{
        let hkey_text = get_hkey_text(hkey);
        let use_ti = is_hklm(hkey);

        let result = {
                let _ti_guard = if use_ti {
                        Some(TrustedInstallerGuard::new()?)
                } else {
                        None
                };

                let create_result =
                        hkey.RegCreateKeyEx(subkey, None, co::REG_OPTION::NON_VOLATILE, co::KEY::WRITE, None);

                let (key, _) = match create_result {
                        Ok(k) => k,
                        Err(source) => {
                                return Err(anyhow!(
                                        "Failed to open Binary registry key: {}\\{}\\{}\nError: {}",
                                        hkey_text,
                                        subkey,
                                        value_name,
                                        source
                                ));
                        }
                };

                let wide_value_name: Vec<u16> = value_name.encode_utf16().chain(std::iter::once(0)).collect();

                unsafe {
                        RegSetValueExW(
                                windows::Win32::System::Registry::HKEY(key.ptr() as *mut _),
                                windows::core::PCWSTR(wide_value_name.as_ptr()),
                                None,
                                REG_BINARY,
                                Some(value),
                        )
                }
        };

        if result.is_err() {
                return Err(anyhow!(
                        "Failed to set Binary registry value: {}\\{}\\{}\nError: {}",
                        hkey_text,
                        subkey,
                        value_name,
                        result.0
                ));
        }

        log_registry(hkey, subkey, value_name, &format!("{:?}", value), "Binary").map_err(|e| {
                anyhow!(
                        "Failed to log Binary change for key: {}\\{}\\{}\nError: {}",
                        hkey_text,
                        subkey,
                        value_name,
                        e
                )
        })?;

        Ok(())
}

pub fn remove_subkey(hkey: &HKEY, subkey: &str) -> Result<()>
{
        let hkey_text = get_hkey_text(hkey);
        let is_hklm = is_hklm(hkey);

        if is_hklm && !trusted_installer::is_trusted_installer() {
                let root = if *hkey == HKEY::LOCAL_MACHINE {
                        RegRoot::HKLM
                } else {
                        return Err(anyhow!("Unsupported root for service delegation"));
                };
                let cmd = ServiceCommand::DeleteRegKey {
                        root,
                        subkey: subkey.to_string(),
                };
                match service_client::send_command(cmd) {
                        Ok(crate::ipc::ServiceResponse::Success) => return Ok(()),
                        Ok(crate::ipc::ServiceResponse::Error(msg)) => return Err(anyhow!("Service error: {}", msg)),
                        Err(e) => return Err(anyhow!("Failed to communicate with W11BoostSvc: {}", e)),
                }
        }

        let use_ti = is_hklm;

        let result = {
                let _ti_guard = if use_ti {
                        Some(TrustedInstallerGuard::new()?)
                } else {
                        None
                };
                hkey.RegDeleteTree(Some(subkey))
        };

        match result {
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
        let is_hklm = is_hklm(hkey);

        if is_hklm && !trusted_installer::is_trusted_installer() {
                let root = if *hkey == HKEY::LOCAL_MACHINE {
                        RegRoot::HKLM
                } else {
                        return Err(anyhow!("Unsupported root for service delegation"));
                };
                let cmd = ServiceCommand::DeleteRegValue {
                        root,
                        subkey: subkey.to_string(),
                        value: value_name.to_string(),
                };
                match service_client::send_command(cmd) {
                        Ok(crate::ipc::ServiceResponse::Success) => return Ok(()),
                        Ok(crate::ipc::ServiceResponse::Error(msg)) => return Err(anyhow!("Service error: {}", msg)),
                        Err(e) => return Err(anyhow!("Failed to communicate with W11BoostSvc: {}", e)),
                }
        }

        let use_ti = is_hklm;

        let result = {
                let _ti_guard = if use_ti {
                        Some(TrustedInstallerGuard::new()?)
                } else {
                        None
                };

                // Note: In original code, RegOpenKeyEx handles FILE_NOT_FOUND carefully.
                let key_res = hkey.RegOpenKeyEx(Some(subkey), co::REG_OPTION::NoValue, co::KEY::SET_VALUE);

                match key_res {
                        Ok(key) => {
                                let r = key.RegDeleteValue(Some(value_name));
                                // Check logic: if RegDeleteValue fails with FILE_NOT_FOUND, it's Ok.
                                r
                        }
                        Err(e) if e == ERROR::FILE_NOT_FOUND => return Ok(()),
                        Err(e) => {
                                return Err(anyhow!(
                                        "Failed to open key for value deletion: {}\\{}\nError: {}",
                                        hkey_text,
                                        subkey,
                                        e
                                ));
                        }
                }
        };

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

fn is_hklm(hkey: &HKEY) -> bool
{
        *hkey == HKEY::LOCAL_MACHINE
}

fn log_registry(hkey: &HKEY, subkey: &str, value_name: &str, value: &str, type_name: &str) -> Result<()>
{
        let hkey_text = get_hkey_text(hkey);
        let log_path = CACHE.lp()?;

        // Create dir check is already done in log_path() which is called by CACHE.lp()

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
                .open(&log_file)
                .with_context(|| format!("Failed to open/create log file: {}", log_file.display()))?;

        file.write_all(log_entry.as_bytes())
                .with_context(|| format!("Failed to write to log file: {}", log_file.display()))?;

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

/// Run a command and return (success, stdout, stderr)
pub fn run_system_command_output(program: &str, args: &[&str]) -> Result<(bool, String, String)>
{
        let output = Command::new(program)
                .args(args)
                .creation_flags(CREATE_NO_WINDOW)
                .output()
                .map_err(|e| anyhow!("Failed to execute {program}.\nError: {e}"))?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Ok((output.status.success(), stdout, stderr))
}

pub fn init_registry_gpo() -> Result<(windows::Win32::System::Registry::HKEY, IGroupPolicyObject)>
{
        unsafe {
                let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
                let gpo: IGroupPolicyObject = CoCreateInstance(&CLSID_GroupPolicyObject, None, CLSCTX_INPROC_SERVER)?;

                gpo.OpenLocalMachineGPO(GPO_OPEN_LOAD_REGISTRY)?;

                let mut hkey = windows::Win32::System::Registry::HKEY::default();
                gpo.GetRegistryKey(GPO_SECTION_MACHINE, &mut hkey)?;

                Ok((hkey, gpo))
        }
}

pub fn save_registry_gpo(hkey: windows::Win32::System::Registry::HKEY, gpo: IGroupPolicyObject) -> Result<()>
{
        let mut snap_guid = GUID::from_u128(0x0f6b957e_509e_11d1_a7cc_0000f87571e3);
        let mut registry_guid = REGISTRY_EXTENSION_GUID;
        unsafe {
                gpo.Save(true, false, &mut registry_guid as *mut _, &mut snap_guid as *mut _)?;
                let _ = RegCloseKey(hkey);
        }

        Ok(())
}

pub fn set_dword_gpo(
        hkey: windows::Win32::System::Registry::HKEY,
        subkey: &str,
        value_name: &str,
        value: u32,
) -> Result<()>
{
        let subkey_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();
        let value_name_wide: Vec<u16> = value_name.encode_utf16().chain(std::iter::once(0)).collect();

        unsafe {
                let mut hsubkey = windows::Win32::System::Registry::HKEY::default();
                RegCreateKeyExW(
                        hkey,
                        PCWSTR(subkey_wide.as_ptr()),
                        Some(0),
                        None,
                        windows::Win32::System::Registry::REG_OPTION_NON_VOLATILE,
                        windows::Win32::System::Registry::KEY_WRITE,
                        None,
                        &mut hsubkey,
                        None,
                )
                .ok()
                .map_err(|e| anyhow!("Failed to create GPO key: {}", e))?;

                RegSetValueExW(
                        hsubkey,
                        PCWSTR(value_name_wide.as_ptr()),
                        Some(0),
                        REG_DWORD,
                        Some(std::slice::from_raw_parts(&value as *const u32 as *const u8, 4)),
                )
                .ok()
                .map_err(|e| anyhow!("Failed to set GPO DWORD value: {}", e))?;

                let _ = RegCloseKey(hsubkey);
        }
        Ok(())
}

pub fn set_string_gpo(
        hkey: windows::Win32::System::Registry::HKEY,
        subkey: &str,
        value_name: &str,
        value: &str,
) -> Result<()>
{
        let subkey_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();
        let value_name_wide: Vec<u16> = value_name.encode_utf16().chain(std::iter::once(0)).collect();
        let value_wide: Vec<u16> = value.encode_utf16().chain(std::iter::once(0)).collect();

        unsafe {
                let mut hsubkey = windows::Win32::System::Registry::HKEY::default();
                RegCreateKeyExW(
                        hkey,
                        PCWSTR(subkey_wide.as_ptr()),
                        Some(0),
                        None,
                        windows::Win32::System::Registry::REG_OPTION_NON_VOLATILE,
                        windows::Win32::System::Registry::KEY_WRITE,
                        None,
                        &mut hsubkey,
                        None,
                )
                .ok()
                .map_err(|e| anyhow!("Failed to create GPO key: {}", e))?;

                let byte_len = (value_wide.len() - 1) * 2 + 2; // includes null terminator

                RegSetValueExW(
                        hsubkey,
                        PCWSTR(value_name_wide.as_ptr()),
                        Some(0),
                        REG_SZ,
                        Some(std::slice::from_raw_parts(value_wide.as_ptr() as *const u8, byte_len)),
                )
                .ok()
                .map_err(|e| anyhow!("Failed to set GPO string value: {}", e))?;

                let _ = RegCloseKey(hsubkey);
        }
        Ok(())
}

pub fn apply_dark_mode_to_window(hwnd_ptr: isize) -> Result<()>
{
        use windows::Win32::Foundation::HWND;
        use windows::Win32::Graphics::Dwm::{DWMWA_USE_IMMERSIVE_DARK_MODE, DwmSetWindowAttribute};

        unsafe {
                let hwnd = HWND(hwnd_ptr as _);
                let dark_mode = 1i32;
                DwmSetWindowAttribute(
                        hwnd,
                        DWMWA_USE_IMMERSIVE_DARK_MODE,
                        &dark_mode as *const _ as *const _,
                        std::mem::size_of::<i32>() as u32,
                )
                .map_err(|e| anyhow!("Failed to set dark mode: {}", e))?;
        }
        Ok(())
}

pub fn run_powershell_command(command: &str) -> Result<()>
{
        run_system_command("powershell.exe", &["-NoProfile", "-Command", command])
}
