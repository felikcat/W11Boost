// TrustedInstaller impersonation for HKLM registry operations

use std::ffi::OsString;
use std::ffi::c_void;
use std::mem::{size_of, zeroed};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::ptr::null_mut;
use std::sync::atomic::{AtomicBool, Ordering};
use windows::Win32::Foundation::{CloseHandle, GetLastError, HANDLE, INVALID_HANDLE_VALUE, LUID, LocalFree};
use windows::Win32::Security::Authorization::ConvertStringSidToSidW;
use windows::Win32::Security::{
        AdjustTokenPrivileges, DuplicateTokenEx, EqualSid, GetTokenInformation, ImpersonateLoggedOnUser,
        LUID_AND_ATTRIBUTES, LookupPrivilegeValueW, RevertToSelf, SE_DEBUG_NAME, SE_IMPERSONATE_NAME,
        SE_PRIVILEGE_ENABLED, SECURITY_ATTRIBUTES, SecurityImpersonation, TOKEN_ADJUST_PRIVILEGES, TOKEN_ALL_ACCESS,
        TOKEN_DUPLICATE, TOKEN_GROUPS, TOKEN_PRIVILEGES, TOKEN_QUERY, TokenGroups, TokenImpersonation,
};
use windows::Win32::System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW, TH32CS_SNAPPROCESS,
};

use windows::Win32::System::Services::{
        CloseServiceHandle, OpenSCManagerW, OpenServiceW, QueryServiceStatusEx, SC_MANAGER_CONNECT,
        SC_STATUS_PROCESS_INFO, SERVICE_QUERY_STATUS, SERVICE_RUNNING, SERVICE_START, SERVICE_STATUS_PROCESS,
        StartServiceW,
};
use windows::Win32::System::Threading::{
        CREATE_UNICODE_ENVIRONMENT, CreateProcessWithTokenW, GetCurrentProcess, LOGON_WITH_PROFILE, OpenProcess,
        OpenProcessToken, PROCESS_INFORMATION, PROCESS_QUERY_LIMITED_INFORMATION, STARTF_USESHOWWINDOW, STARTUPINFOW,
};
use windows::Win32::UI::WindowsAndMessaging::SW_SHOW;
use windows::core::{PCWSTR, PWSTR, w};

const SLEEP_INTERVAL: u32 = 100;
const MAX_LOOPS: u32 = 50; // 5 seconds total

static PRIVILEGES_ENABLED: AtomicBool = AtomicBool::new(false);

/// Enable debug and impersonate privileges (only needs to be done once)
fn ensure_privileges() -> bool
{
        if PRIVILEGES_ENABLED.load(Ordering::Relaxed) {
                return true;
        }

        unsafe {
                let mut token: HANDLE = HANDLE::default();
                if OpenProcessToken(
                        GetCurrentProcess(),
                        TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,
                        &raw mut token,
                )
                .is_err()
                {
                        return false;
                }

                let mut luid_debug: LUID = zeroed();
                let mut luid_impersonate: LUID = zeroed();

                if LookupPrivilegeValueW(PCWSTR::null(), SE_DEBUG_NAME, &raw mut luid_debug).is_err() {
                        let _ = CloseHandle(token);
                        return false;
                }
                if LookupPrivilegeValueW(PCWSTR::null(), SE_IMPERSONATE_NAME, &raw mut luid_impersonate).is_err() {
                        let _ = CloseHandle(token);
                        return false;
                }

                let mut tp: TOKEN_PRIVILEGES = zeroed();
                tp.PrivilegeCount = 2;
                tp.Privileges[0] = LUID_AND_ATTRIBUTES {
                        Luid: luid_debug,
                        Attributes: SE_PRIVILEGE_ENABLED,
                };
                let privileges_ptr = tp.Privileges.as_mut_ptr();
                *privileges_ptr.add(1) = LUID_AND_ATTRIBUTES {
                        Luid: luid_impersonate,
                        Attributes: SE_PRIVILEGE_ENABLED,
                };

                let result = AdjustTokenPrivileges(token, false, Some(&raw const tp), 0, None, None);
                let err = GetLastError();
                let _ = CloseHandle(token);

                if result.is_err() {
                        crate::common::log_debug(
                                "ti",
                                &format!("ensure_privileges: AdjustTokenPrivileges FAILED error={:?}", err),
                        );
                        return false;
                }

                if err.0 != 0 {
                        // ERROR_NOT_ALL_ASSIGNED or similar
                        crate::common::log_debug(
                                "ti",
                                &format!(
                                        "ensure_privileges: AdjustTokenPrivileges OK but GetLastError={:?} (Not all assigned?)",
                                        err
                                ),
                        );
                        // If not all assigned, we failed to get SeDebugPrivilege likely
                        if err == windows::Win32::Foundation::ERROR_NOT_ALL_ASSIGNED {
                                return false;
                        }
                }
                PRIVILEGES_ENABLED.store(true, Ordering::Relaxed);
                true
        }
}

/// Get PID of a process by name
fn get_pid_from_name(process_name: &str) -> Option<u32>
{
        unsafe {
                let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).ok()?;
                if snapshot == INVALID_HANDLE_VALUE {
                        return None;
                }

                let mut entry: PROCESSENTRY32W = zeroed();
                entry.dwSize = size_of::<PROCESSENTRY32W>() as u32;

                if Process32FirstW(snapshot, &raw mut entry).is_err() {
                        let _ = CloseHandle(snapshot);
                        return None;
                }

                loop {
                        let name_len = entry
                                .szExeFile
                                .iter()
                                .position(|&c| c == 0)
                                .unwrap_or(entry.szExeFile.len());
                        let name = OsString::from_wide(&entry.szExeFile[..name_len]);

                        if name.to_string_lossy().eq_ignore_ascii_case(process_name) {
                                let _ = CloseHandle(snapshot);
                                return Some(entry.th32ProcessID);
                        }

                        if Process32NextW(snapshot, &raw mut entry).is_err() {
                                break;
                        }
                }

                let _ = CloseHandle(snapshot);
                None
        }
}

/// Create a duplicate token from a process ID
fn create_token_from_pid(pid: u32) -> Option<HANDLE>
{
        fn log_pid(msg: &str)
        {
                crate::common::log_debug("ti", &format!("create_token: {}", msg));
        }

        unsafe {
                let process = match OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) {
                        Ok(p) => p,
                        Err(e) => {
                                log_pid(&format!("OpenProcess({}) FAILED, error={:?}", pid, e));
                                return None;
                        }
                };

                let mut token: HANDLE = HANDLE::default();
                if let Err(e) = OpenProcessToken(process, TOKEN_DUPLICATE, &raw mut token) {
                        log_pid(&format!("OpenProcessToken FAILED, error={:?}", e));
                        let _ = CloseHandle(process);
                        return None;
                }

                let mut dup_token: HANDLE = HANDLE::default();
                let attrs = SECURITY_ATTRIBUTES {
                        nLength: size_of::<SECURITY_ATTRIBUTES>() as u32,
                        lpSecurityDescriptor: null_mut(),
                        bInheritHandle: false.into(),
                };

                let result = DuplicateTokenEx(
                        token,
                        TOKEN_ALL_ACCESS,
                        Some(&raw const attrs),
                        SecurityImpersonation,
                        TokenImpersonation,
                        &raw mut dup_token,
                );

                let _ = CloseHandle(token);
                let _ = CloseHandle(process);

                if result.is_err() {
                        log_pid(&format!(
                                "DuplicateTokenEx FAILED, error={:?}",
                                windows::Win32::Foundation::GetLastError()
                        ));
                        return None;
                }
                Some(dup_token)
        }
}

/// Impersonate SYSTEM via winlogon.exe token
fn impersonate_system() -> bool
{
        fn log_sys(msg: &str)
        {
                crate::common::log_debug("ti", &format!("impersonate_system: {}", msg));
        }

        let Some(winlogon_pid) = get_pid_from_name("winlogon.exe") else {
                log_sys("get_pid_from_name(winlogon.exe) FAILED");
                return false;
        };
        log_sys(&format!("winlogon.exe pid={}", winlogon_pid));

        let Some(token) = create_token_from_pid(winlogon_pid) else {
                log_sys("create_token_from_pid(winlogon) FAILED");
                return false;
        };
        log_sys("create_token_from_pid OK");

        unsafe {
                let result = ImpersonateLoggedOnUser(token);
                let _ = CloseHandle(token);
                if result.is_err() {
                        let err = windows::Win32::Foundation::GetLastError();
                        log_sys(&format!("ImpersonateLoggedOnUser FAILED, error={:?}", err));
                        return false;
                }
                log_sys("ImpersonateLoggedOnUser OK");
                true
        }
}

/// Start `TrustedInstaller` service and get its PID
fn start_ti_service_and_get_pid() -> Option<u32>
{
        unsafe {
                let scm = OpenSCManagerW(PCWSTR::null(), PCWSTR::null(), SC_MANAGER_CONNECT).ok()?;

                let Ok(service) = OpenServiceW(scm, w!("TrustedInstaller"), SERVICE_START | SERVICE_QUERY_STATUS)
                else {
                        let _ = CloseServiceHandle(scm);
                        return None;
                };

                let mut loops = MAX_LOOPS;
                let mut pid: Option<u32> = None;

                while loops > 0 {
                        let mut buffer = [0u8; size_of::<SERVICE_STATUS_PROCESS>()];
                        let mut bytes_needed: u32 = 0;

                        if QueryServiceStatusEx(
                                service,
                                SC_STATUS_PROCESS_INFO,
                                Some(&mut buffer),
                                &raw mut bytes_needed,
                        )
                        .is_err()
                        {
                                std::thread::sleep(std::time::Duration::from_millis(u64::from(SLEEP_INTERVAL)));
                                loops -= 1;
                                continue;
                        }

                        let ssp: &SERVICE_STATUS_PROCESS = &*(buffer.as_ptr().cast());
                        if ssp.dwCurrentState == SERVICE_RUNNING {
                                pid = Some(ssp.dwProcessId);
                                break;
                        }

                        if ssp.dwCurrentState.0 == 1 {
                                // SERVICE_STOPPED
                                if StartServiceW(service, None).is_err() {
                                        break;
                                }
                        }
                        std::thread::sleep(std::time::Duration::from_millis(u64::from(SLEEP_INTERVAL)));
                        loops -= 1;
                }

                let _ = CloseServiceHandle(service);
                let _ = CloseServiceHandle(scm);
                pid
        }
}

/// Impersonate `TrustedInstaller` for the current thread.
/// Returns true if impersonation succeeded.
/// Call `revert_to_self()` after the privileged operation.
#[must_use]
pub fn impersonate_trusted_installer() -> bool
{
        if !ensure_privileges() {
                return false;
        }

        if !impersonate_system() {
                return false;
        }

        let Some(ti_pid) = start_ti_service_and_get_pid() else {
                let _ = unsafe { RevertToSelf() };
                return false;
        };

        let Some(ti_token) = create_token_from_pid(ti_pid) else {
                let _ = unsafe { RevertToSelf() };
                return false;
        };

        unsafe {
                let result = ImpersonateLoggedOnUser(ti_token);
                let _ = CloseHandle(ti_token);
                result.is_ok()
        }
}
/// Revert impersonation back to the process token
pub fn revert_to_self()
{
        let _ = unsafe { RevertToSelf() };
}

/// Execute a closure while impersonating `TrustedInstaller`.
/// Automatically reverts impersonation after the closure completes.
pub fn with_trusted_installer<F, R>(f: F) -> Option<R>
where
        F: FnOnce() -> R,
{
        if !impersonate_trusted_installer() {
                return None;
        }
        let result = f();
        revert_to_self();
        Some(result)
}

/// RAII guard for TrustedInstaller impersonation.
/// Reverts to self when dropped.
pub struct TrustedInstallerGuard
{
        active: bool,
}

impl TrustedInstallerGuard
{
        /// Attempts to impersonate TrustedInstaller.
        pub fn new() -> anyhow::Result<Self>
        {
                if impersonate_trusted_installer() {
                        Ok(Self { active: true })
                } else {
                        Err(anyhow::anyhow!("Failed to impersonate TrustedInstaller"))
                }
        }
}

impl Drop for TrustedInstallerGuard
{
        fn drop(&mut self)
        {
                if self.active {
                        revert_to_self();
                }
        }
}

pub fn is_trusted_installer() -> bool
{
        unsafe {
                let mut token: HANDLE = HANDLE::default();
                if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &raw mut token).is_err() {
                        return false;
                }

                // First check if we are running as SYSTEM (S-1-5-18)
                // This is a prerequisite for TrustedInstaller (usually) and sufficient for most operations.
                // However, user specifically asked for TrustedInstaller check.
                // But wait, if we are SYSTEM, we might loop if we strictly require TI group.
                // Let's check for TI group specifically as requested by the looped behavior.

                let mut len = 0;
                let _ = GetTokenInformation(token, TokenGroups, None, 0, &mut len);
                if len == 0 {
                        let _ = CloseHandle(token);
                        return false;
                }

                let mut buf = vec![0u8; len as usize];
                if GetTokenInformation(token, TokenGroups, Some(buf.as_mut_ptr() as *mut c_void), len, &mut len)
                        .is_err()
                {
                        let _ = CloseHandle(token);
                        return false;
                }

                let token_groups = &*(buf.as_ptr() as *const TOKEN_GROUPS);
                let groups = std::slice::from_raw_parts(token_groups.Groups.as_ptr(), token_groups.GroupCount as usize);

                // TrustedInstaller SID: S-1-5-80-956008885-3418522649-1831038044-1853292631-2271478464
                let ti_sid_str = w!("S-1-5-80-956008885-3418522649-1831038044-1853292631-2271478464");
                let mut ti_sid = windows::Win32::Security::PSID::default();

                if ConvertStringSidToSidW(ti_sid_str, &mut ti_sid).is_ok() {
                        // Check if any group matches TrustedInstaller SID
                        let mut found = false;
                        for group in groups {
                                if EqualSid(group.Sid, ti_sid).is_ok() {
                                        found = true;
                                        break;
                                }
                        }

                        let _ = LocalFree(Some(windows::Win32::Foundation::HLOCAL(ti_sid.0 as *mut c_void)));
                        let _ = CloseHandle(token);
                        return found;
                }

                let _ = CloseHandle(token);
                false
        }
}

pub fn spawn_process_as_trusted_installer(app_path: &std::path::Path, cmd_line: &str) -> bool
{
        fn log_ti(msg: &str)
        {
                crate::common::log_debug("ti", msg);
        }

        log_ti(&format!(
                "spawn_process_as_trusted_installer() called for {:?} with args '{}'",
                app_path, cmd_line
        ));

        if !ensure_privileges() {
                log_ti("ensure_privileges() FAILED");
                return false;
        }

        if !impersonate_system() {
                log_ti("impersonate_system() FAILED");
                return false;
        }

        let Some(ti_pid) = start_ti_service_and_get_pid() else {
                log_ti("start_ti_service_and_get_pid() FAILED");
                let _ = unsafe { RevertToSelf() };
                return false;
        };
        log_ti(&format!("start_ti_service_and_get_pid() OK, pid={}", ti_pid));

        let Some(ti_token) = create_token_from_pid(ti_pid) else {
                log_ti("create_token_from_pid() FAILED");
                let _ = unsafe { RevertToSelf() };
                return false;
        };
        log_ti("create_token_from_pid() OK");

        // Convert to wide string for Windows API
        // Note: cmd_line for CreateProcess usually requires the application name as the first token.
        // However, CreateProcessWithTokenW behavior can be subtle.
        // Ideally cmd_line should be "AppPath Args".

        let mut full_cmd_line = format!("\"{}\" {}", app_path.display(), cmd_line);
        // If cmd_line is empty, just app path
        if cmd_line.trim().is_empty() {
                full_cmd_line = format!("\"{}\"", app_path.display());
        }

        let mut cmd_line_wide: Vec<u16> = full_cmd_line.encode_utf16().chain(std::iter::once(0)).collect();
        let app_name_wide: Vec<u16> = app_path.as_os_str().encode_wide().chain(std::iter::once(0)).collect();

        let mut startup_info: STARTUPINFOW = unsafe { zeroed() };
        startup_info.cb = size_of::<STARTUPINFOW>() as u32;
        startup_info.dwFlags = STARTF_USESHOWWINDOW;
        startup_info.wShowWindow = SW_SHOW.0 as u16;

        let mut process_info: PROCESS_INFORMATION = unsafe { zeroed() };

        // SYSTEM cannot access UNC paths (\vmware-host\...), so we MUST set a local CWD.
        // We use the parent directory of the executable.
        let working_dir = app_path.parent().unwrap_or(app_path);
        let working_dir_wide: Vec<u16> = working_dir
                .as_os_str()
                .encode_wide()
                .chain(std::iter::once(0))
                .collect();

        unsafe {
                let result = CreateProcessWithTokenW(
                        ti_token,
                        LOGON_WITH_PROFILE,
                        PCWSTR(app_name_wide.as_ptr()),
                        Some(PWSTR(cmd_line_wide.as_mut_ptr())),
                        CREATE_UNICODE_ENVIRONMENT,
                        None,                              // Environment
                        PCWSTR(working_dir_wide.as_ptr()), // Current directory
                        &raw const startup_info,
                        &raw mut process_info,
                );

                let _ = CloseHandle(ti_token);
                let _ = RevertToSelf(); // Important to revert impersonation of SYSTEM

                if result.is_err() {
                        let err = windows::Win32::Foundation::GetLastError();
                        log_ti(&format!("CreateProcessWithTokenW FAILED, error={:?}", err));
                        return false;
                }

                log_ti(&format!(
                        "CreateProcessWithTokenW OK, new pid={}",
                        process_info.dwProcessId
                ));
                let _ = CloseHandle(process_info.hProcess);
                let _ = CloseHandle(process_info.hThread);
                true
        }
}

pub fn relaunch_as_trusted_installer() -> bool
{
        let current_exe = std::env::current_exe().unwrap_or_default();
        let cmd_line_str = std::env::args().skip(1).collect::<Vec<_>>().join(" "); // Skip arg0
        spawn_process_as_trusted_installer(&current_exe, &cmd_line_str)
}

#[cfg(test)]
mod tests
{
        use super::*;
        use std::ffi::c_void;
        use windows::Win32::Security::{GetTokenInformation, LookupAccountSidW, SID_NAME_USE, TOKEN_USER, TokenUser};
        use windows::Win32::System::Threading::{GetCurrentThread, OpenThreadToken};
        use windows::core::PWSTR;

        fn get_current_user_name() -> Option<String>
        {
                unsafe {
                        let mut token = HANDLE::default();
                        // OpenThreadToken checks for thread token. if failing, likely not impersonating on this thread.
                        if OpenThreadToken(GetCurrentThread(), TOKEN_QUERY, true, &mut token).is_err() {
                                return None;
                        }

                        let mut len = 0;
                        let _ = GetTokenInformation(token, TokenUser, None, 0, &mut len);
                        if len == 0 {
                                let _ = CloseHandle(token);
                                return None;
                        }

                        let mut buf = vec![0u8; len as usize];
                        if GetTokenInformation(token, TokenUser, Some(buf.as_mut_ptr() as *mut c_void), len, &mut len)
                                .is_err()
                        {
                                let _ = CloseHandle(token);
                                return None;
                        }

                        let token_user = &*(buf.as_ptr() as *const TOKEN_USER);
                        let sid = token_user.User.Sid;

                        let mut name_buf = [0u16; 256];
                        let mut domain_buf = [0u16; 256];
                        let mut name_len = 256;
                        let mut domain_len = 256;
                        let mut use_out = SID_NAME_USE::default();

                        let res = if LookupAccountSidW(
                                PCWSTR::null(),
                                sid,
                                Some(PWSTR(name_buf.as_mut_ptr())),
                                &mut name_len,
                                Some(PWSTR(domain_buf.as_mut_ptr())),
                                &mut domain_len,
                                &mut use_out,
                        )
                        .is_ok()
                        {
                                let name = String::from_utf16_lossy(&name_buf[..name_len as usize]);
                                let domain = String::from_utf16_lossy(&domain_buf[..domain_len as usize]);
                                Some(format!("{}\\{}", domain, name))
                        } else {
                                None
                        };

                        let _ = CloseHandle(token);
                        res
                }
        }

        #[test]
        fn test_impersonation()
        {
                if !ensure_privileges() {
                        println!("Skipping test: Not running as Admin or failed to get privileges.");
                        return;
                }

                let result = with_trusted_installer(|| get_current_user_name());

                match result {
                        Some(Some(user)) => {
                                println!("Impersonated user: {}", user);
                                let user_lower = user.to_lowercase();
                                assert!(
                                        user_lower.contains("trustedinstaller") || user_lower.contains("system"),
                                        "Expected TrustedInstaller or SYSTEM, got {}",
                                        user
                                );
                        }
                        Some(None) => panic!("Failed to get user name while impersonating"),
                        None => panic!("Failed to impersonate TrustedInstaller"),
                }
        }
}
