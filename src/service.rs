use crate::ipc::{RegRoot, ServiceCommand, ServiceResponse};

use anyhow::{Context, Result};
use std::ffi::c_void;
use std::mem::size_of;
use windows::Win32::Foundation::{HANDLE, INVALID_HANDLE_VALUE};
use windows::Win32::Security::{
        InitializeSecurityDescriptor, PSECURITY_DESCRIPTOR, SECURITY_ATTRIBUTES, SECURITY_DESCRIPTOR,
        SetSecurityDescriptorDacl,
};
use windows::Win32::Storage::FileSystem::PIPE_ACCESS_DUPLEX;
use windows::Win32::System::Pipes::{
        ConnectNamedPipe, CreateNamedPipeW, PIPE_READMODE_MESSAGE, PIPE_TYPE_MESSAGE, PIPE_UNLIMITED_INSTANCES,
        PIPE_WAIT,
};
use windows::core::PCWSTR;

const PIPE_NAME: &str = r"\\.\pipe\W11BoostSvc";

pub fn log_debug(msg: &str)
{
        crate::common::log_debug("service", msg);
}

pub fn run() -> Result<()>
{
        log_debug("Service run() called");
        // We assume we are spawned as TI by the client.
        // Even if not, we try to run.
        log_debug("Starting server loop...");
        run_server()
}

fn run_server() -> Result<()>
{
        let pipe_name_wide: Vec<u16> = PIPE_NAME.encode_utf16().chain(std::iter::once(0)).collect();

        unsafe {
                let mut sd = SECURITY_DESCRIPTOR::default();
                let p_sd = PSECURITY_DESCRIPTOR(&raw mut sd as *mut _ as *mut c_void);

                // Initialize Security Descriptor with NULL DACL (Allow Everyone)
                InitializeSecurityDescriptor(p_sd, 1).ok(); // 1 = SECURITY_DESCRIPTOR_REVISION
                SetSecurityDescriptorDacl(p_sd, true, None, false).ok();

                let sa = SECURITY_ATTRIBUTES {
                        nLength: size_of::<SECURITY_ATTRIBUTES>() as u32,
                        lpSecurityDescriptor: p_sd.0 as *mut c_void,
                        bInheritHandle: false.into(),
                };

                let pipe_handle = CreateNamedPipeW(
                        PCWSTR(pipe_name_wide.as_ptr()),
                        PIPE_ACCESS_DUPLEX,
                        PIPE_TYPE_MESSAGE | PIPE_READMODE_MESSAGE | PIPE_WAIT,
                        PIPE_UNLIMITED_INSTANCES,
                        4096,      // Out buffer
                        4096,      // In buffer
                        0,         // Default timeout
                        Some(&sa), // Security attributes
                );

                if pipe_handle == INVALID_HANDLE_VALUE {
                        let err = windows::Win32::Foundation::GetLastError();
                        log_debug(&format!("Failed to create named pipe initially. Error: {:?}", err));
                        return Err(anyhow::anyhow!("Failed to create named pipe: {:?}", err));
                }

                log_debug("Named pipe created successfully. Waiting for client connection...");

                // Wait for client connection
                // This blocks until a client connects or an error occurs
                let connected = ConnectNamedPipe(pipe_handle, None).is_ok()
                        || windows::Win32::Foundation::GetLastError()
                                == windows::Win32::Foundation::ERROR_PIPE_CONNECTED;

                if connected {
                        log_debug("Client connected, entering command loop.");
                        handle_client(pipe_handle);
                        log_debug("Client disconnected, service exiting.");
                } else {
                        let err = windows::Win32::Foundation::GetLastError();
                        log_debug(&format!("ConnectNamedPipe failed. Error: {:?}", err));
                }

                // CloseHandle is not strictly necessary as process exit cleans up,
                // but good practice if we were staying alive.
                // windows::Win32::Foundation::CloseHandle(pipe_handle);
        }

        Ok(())
}

fn handle_client(pipe: HANDLE)
{
        // Read loop for this connection
        loop {
                let Ok(msg_bytes) = read_message(pipe) else {
                        break; // Read error
                };

                if msg_bytes.is_empty() {
                        break;
                } // EOF/Disconnect

                let response = match serde_json::from_slice::<ServiceCommand>(&msg_bytes) {
                        Ok(cmd) => process_command(cmd),
                        Err(e) => ServiceResponse::Error(format!("Invalid command JSON: {}", e)),
                };

                if send_response(pipe, &response).is_err() {
                        break; // Send failed, disconnect
                }
        }
}

fn process_command(cmd: ServiceCommand) -> ServiceResponse
{
        match cmd {
                ServiceCommand::Ping => ServiceResponse::Success,
                ServiceCommand::WriteRegDword {
                        root,
                        subkey,
                        value,
                        data,
                } => {
                        let hkey = match root {
                                RegRoot::HKLM => windows::Win32::System::Registry::HKEY_LOCAL_MACHINE,
                                RegRoot::HKCU => windows::Win32::System::Registry::HKEY_CURRENT_USER,
                                RegRoot::HKCR => windows::Win32::System::Registry::HKEY_CLASSES_ROOT,
                                RegRoot::HKU => windows::Win32::System::Registry::HKEY_USERS,
                        };

                        let ws_hkey = unsafe { winsafe::HKEY::from_ptr(hkey.0 as *mut _) };

                        if let Err(e) = crate::common::set_dword(&ws_hkey, &subkey, &value, data) {
                                ServiceResponse::Error(e.to_string())
                        } else {
                                ServiceResponse::Success
                        }
                }
                ServiceCommand::WriteRegString {
                        root,
                        subkey,
                        value,
                        data,
                } => {
                        let hkey = match root {
                                RegRoot::HKLM => windows::Win32::System::Registry::HKEY_LOCAL_MACHINE,
                                RegRoot::HKCU => windows::Win32::System::Registry::HKEY_CURRENT_USER,
                                RegRoot::HKCR => windows::Win32::System::Registry::HKEY_CLASSES_ROOT,
                                RegRoot::HKU => windows::Win32::System::Registry::HKEY_USERS,
                        };
                        let ws_hkey = unsafe { winsafe::HKEY::from_ptr(hkey.0 as *mut _) };

                        if let Err(e) = crate::common::set_string(&ws_hkey, &subkey, &value, &data) {
                                ServiceResponse::Error(e.to_string())
                        } else {
                                ServiceResponse::Success
                        }
                }
                ServiceCommand::DeleteRegValue { root, subkey, value } => {
                        let hkey = match root {
                                RegRoot::HKLM => windows::Win32::System::Registry::HKEY_LOCAL_MACHINE,
                                RegRoot::HKCU => windows::Win32::System::Registry::HKEY_CURRENT_USER,
                                RegRoot::HKCR => windows::Win32::System::Registry::HKEY_CLASSES_ROOT,
                                RegRoot::HKU => windows::Win32::System::Registry::HKEY_USERS,
                        };
                        let ws_hkey = unsafe { winsafe::HKEY::from_ptr(hkey.0 as *mut _) };

                        if let Err(e) = crate::common::delete_value(&ws_hkey, &subkey, &value) {
                                ServiceResponse::Error(e.to_string())
                        } else {
                                ServiceResponse::Success
                        }
                }
                ServiceCommand::DeleteRegKey { root, subkey } => {
                        let hkey = match root {
                                RegRoot::HKLM => windows::Win32::System::Registry::HKEY_LOCAL_MACHINE,
                                RegRoot::HKCU => windows::Win32::System::Registry::HKEY_CURRENT_USER,
                                RegRoot::HKCR => windows::Win32::System::Registry::HKEY_CLASSES_ROOT,
                                RegRoot::HKU => windows::Win32::System::Registry::HKEY_USERS,
                        };
                        let ws_hkey = unsafe { winsafe::HKEY::from_ptr(hkey.0 as *mut _) };

                        if let Err(e) = crate::common::remove_subkey(&ws_hkey, &subkey) {
                                ServiceResponse::Error(e.to_string())
                        } else {
                                ServiceResponse::Success
                        }
                }

                _ => ServiceResponse::Error("Not implemented".to_string()),
        }
}

fn read_message(pipe: HANDLE) -> Result<Vec<u8>>
{
        let mut buf = [0u8; 4096];
        let mut bytes_read = 0u32;
        unsafe {
                windows::Win32::Storage::FileSystem::ReadFile(
                        pipe,
                        Some(&mut buf),
                        Some(&mut bytes_read as *mut u32),
                        None,
                )
                .ok()
                .context("ReadFile failed")?;
        }
        Ok(buf[..bytes_read as usize].to_vec())
}

fn send_response(pipe: HANDLE, resp: &ServiceResponse) -> Result<()>
{
        let bytes = serde_json::to_vec(resp)?;
        let mut written = 0u32;
        unsafe {
                windows::Win32::Storage::FileSystem::WriteFile(
                        pipe,
                        Some(&bytes),
                        Some(&mut written as *mut u32),
                        None,
                )
                .ok()
                .context("WriteFile failed")?;
        }
        Ok(())
}
