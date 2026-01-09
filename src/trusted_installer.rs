// TrustedInstaller impersonation for HKLM registry operations

use std::ffi::OsString;
use std::mem::{size_of, zeroed};
use std::os::windows::ffi::OsStringExt;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicBool, Ordering};
use windows::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE, LUID};
use windows::Win32::Security::{
        AdjustTokenPrivileges, DuplicateTokenEx, ImpersonateLoggedOnUser, LUID_AND_ATTRIBUTES, LookupPrivilegeValueW,
        RevertToSelf, SE_DEBUG_NAME, SE_IMPERSONATE_NAME, SE_PRIVILEGE_ENABLED, SECURITY_ATTRIBUTES,
        SecurityImpersonation, TOKEN_ADJUST_PRIVILEGES, TOKEN_ALL_ACCESS, TOKEN_DUPLICATE, TOKEN_PRIVILEGES,
        TOKEN_QUERY, TokenImpersonation,
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
        GetCurrentProcess, OpenProcess, OpenProcessToken, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ,
};
use windows::core::{PCWSTR, w};

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
                let _ = CloseHandle(token);

                if result.is_ok() {
                        PRIVILEGES_ENABLED.store(true, Ordering::Relaxed);
                        true
                } else {
                        false
                }
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
        unsafe {
                let process = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid).ok()?;

                let mut token: HANDLE = HANDLE::default();
                if OpenProcessToken(process, TOKEN_DUPLICATE, &raw mut token).is_err() {
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

                if result.is_ok() { Some(dup_token) } else { None }
        }
}

/// Impersonate SYSTEM via winlogon.exe token
fn impersonate_system() -> bool
{
        let Some(winlogon_pid) = get_pid_from_name("winlogon.exe") else {
                return false;
        };

        let Some(token) = create_token_from_pid(winlogon_pid) else {
                return false;
        };

        unsafe {
                let result = ImpersonateLoggedOnUser(token);
                let _ = CloseHandle(token);
                result.is_ok()
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
                        .is_ok()
                        {
                                let ssp: &SERVICE_STATUS_PROCESS = &*(buffer.as_ptr().cast());
                                if ssp.dwCurrentState == SERVICE_RUNNING {
                                        pid = Some(ssp.dwProcessId);
                                        break;
                                } else if ssp.dwCurrentState.0 == 1 {
                                        // SERVICE_STOPPED
                                        if StartServiceW(service, None).is_err() {
                                                break;
                                        }
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
