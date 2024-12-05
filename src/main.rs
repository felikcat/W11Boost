pub mod common;
mod gui;
use fltk::dialog;
use gui::draw_gui;
use std::error::Error;
use std::os::raw::c_void;
use std::ptr::null_mut;
use std::time::Duration;
use std::{mem, thread};
use windows::Win32::System::Services::{
    CloseServiceHandle, OpenSCManagerW, OpenServiceW, QueryServiceStatusEx, SC_MANAGER_CONNECT,
    SC_STATUS_PROCESS_INFO, SERVICE_QUERY_STATUS, SERVICE_RUNNING, SERVICE_START,
    SERVICE_STATUS_PROCESS, SERVICE_STOPPED, StartServiceW,
};
use windows::core::w;
use windows_sys::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE};
use windows_sys::Win32::Security::{
    DuplicateTokenEx, ImpersonateLoggedOnUser, SECURITY_ATTRIBUTES, SecurityImpersonation,
    TOKEN_ALL_ACCESS, TOKEN_DUPLICATE, TokenImpersonation,
};
use windows_sys::Win32::System::SystemServices::MAXIMUM_ALLOWED;
use windows_sys::Win32::System::Threading::{
    CREATE_UNICODE_ENVIRONMENT, CreateProcessWithTokenW, LOGON_WITH_PROFILE, OpenProcessToken,
    PROCESS_INFORMATION, STARTUPINFOW,
};
use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcess};
use winsafe::{self as ws, co, prelude::*};

fn get_pid_from_process_name(process_name: &str) -> Result<u32, Box<dyn Error>> {
    let mut pid = 0;
    let mut process = ws::HPROCESSLIST::CreateToolhelp32Snapshot(
        co::TH32CS::SNAPPROCESS,
        Some(0),
    ).expect("Failed to create process list snapshot");

    let mut pe32 = ws::PROCESSENTRY32::default();
    process.Process32First(&mut pe32).expect("Failed to get first process");

    while pe32.szExeFile() != process_name {
        process.Process32Next(&mut pe32)?;
        pid = pe32.th32ProcessID;
    }

    Ok(pid)
}

fn impersonate_as_system() -> Result<bool, Box<dyn Error>> {
    let mut success = false;
    let pid = get_pid_from_process_name("winlogon.exe").unwrap();

    let token = create_access_token_from_pid(pid).unwrap();

    unsafe {
        if ImpersonateLoggedOnUser(token) == 0 {
            eprintln!("Failed to impersonate as SYSTEM");
        } else {
            success = true;
        }
        CloseHandle(token);
    }

    Ok(success)
}

fn create_access_token_from_pid(process_id: u32) -> Result<*mut c_void, Box<dyn Error>> {
    let mut dup_token = INVALID_HANDLE_VALUE;
    unsafe {
        let process = OpenProcess(MAXIMUM_ALLOWED, 0, process_id);
        if !process.is_null() {
            let mut token = INVALID_HANDLE_VALUE;
            OpenProcessToken(process, TOKEN_DUPLICATE, &mut token);

            let attributes = SECURITY_ATTRIBUTES {
                nLength: size_of::<SECURITY_ATTRIBUTES>() as u32,
                lpSecurityDescriptor: null_mut(),
                bInheritHandle: 0,
            };

            DuplicateTokenEx(
                token,
                TOKEN_ALL_ACCESS,
                &attributes,
                SecurityImpersonation,
                TokenImpersonation,
                &mut dup_token,
            );
        }
    }

    Ok(dup_token)
}

fn start_trusted_installer_service() -> Result<u32, Box<dyn Error>> {
    const SLEEP_INTERVAL: u64 = 100;
    const MAX_ATTEMPTS: u32 = 50; // 50 * 100ms = 5 seconds

    let mut attempts = MAX_ATTEMPTS;
    let mut process_id = 0;

    let scm_handle = unsafe { OpenSCManagerW(None, None, SC_MANAGER_CONNECT) }
        .expect("Failed to connect to Service Control Manager");

    let service_handle = unsafe {
        OpenServiceW(
            scm_handle,
            w!("TrustedInstaller"),
            SERVICE_START | SERVICE_QUERY_STATUS,
        )
    }
    .expect("Failed to open TrustedInstaller service");

    while attempts > 0 {
        unsafe {
            let mut buffer: [u8; mem::size_of::<SERVICE_STATUS_PROCESS>()] =
                [0; mem::size_of::<SERVICE_STATUS_PROCESS>()];
            QueryServiceStatusEx(
                service_handle,
                SC_STATUS_PROCESS_INFO,
                Some(&mut buffer),
                &mut (mem::size_of::<SERVICE_STATUS_PROCESS>() as u32),
            )
            .expect("Failed to query the statuses of services");
            let status: SERVICE_STATUS_PROCESS = mem::transmute(buffer);

            if status.dwCurrentState == SERVICE_RUNNING {
                process_id = status.dwProcessId;
                break;
            } else if status.dwCurrentState == SERVICE_STOPPED {
                StartServiceW(service_handle, None)
                    .expect("Failed to start TrustedInstaller service");
            } else {
                eprintln!(
                    "TrustedInstaller service is in an unexpected state: {:?}",
                    status.dwCurrentState
                );
            }
        }

        thread::sleep(Duration::from_millis(SLEEP_INTERVAL));
        attempts -= 1;
    }

    // Cleanup.
    unsafe {
        CloseServiceHandle(service_handle).expect("Failed to close service_handle");
        CloseServiceHandle(scm_handle).expect("Failed to close scm_handle");
    }

    Ok(process_id)
}

fn main() -> Result<(), Box<dyn Error>> {
    impersonate_as_system()?;

    let service_id = start_trusted_installer_service().unwrap();
    create_access_token_from_pid(service_id)?;

    // Get process name for the currently ran process.
    let mut process = ws::HPROCESSLIST::CreateToolhelp32Snapshot(
        co::TH32CS::SNAPPROCESS,
        Some(ws::GetCurrentProcessId()),
    )?;

    let mut pe32 = ws::PROCESSENTRY32::default();
    process.Process32First(&mut pe32)?;

    unsafe {
        let process_handle = GetCurrentProcess();
        let mut token_handle = create_access_token_from_pid(service_id).unwrap();

        let startup_info = STARTUPINFOW {
            cb: mem::size_of::<STARTUPINFOW>() as u32,
            ..mem::zeroed()
        };
        let mut process_info = PROCESS_INFORMATION { ..mem::zeroed() };

        OpenProcessToken(process_handle, MAXIMUM_ALLOWED, &mut token_handle);

        let exe_path = widestring::U16CString::from_str(pe32.szExeFile()).unwrap();

        CreateProcessWithTokenW(
            token_handle,
            LOGON_WITH_PROFILE,
            exe_path.as_ptr(),
            null_mut(),
            CREATE_UNICODE_ENVIRONMENT,
            null_mut(),
            null_mut(),
            &startup_info,
            &mut process_info,
        );
    }

    match draw_gui() {
        Ok(_) => println!("draw_gui() exited successfully"),
        Err(e) => dialog::alert(0, 0, &format!("W11Boost -> draw_gui() failed: {}", e)),
    }

    Ok(())
}
