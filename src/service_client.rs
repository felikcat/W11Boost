use crate::ipc::{ServiceCommand, ServiceResponse};
use anyhow::{Context, Result, anyhow};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::os::windows::process::CommandExt;
use std::sync::Mutex;

use windows::Win32::System::Pipes::WaitNamedPipeW;
use windows::core::PCWSTR;

const PIPE_NAME: &str = r"\\.\pipe\W11BoostSvc";

/// Global mutex to hold the persistent pipe connection
static PIPE_CONNECTION: Mutex<Option<std::fs::File>> = Mutex::new(None);

fn log_client(msg: &str)
{
        crate::common::log_debug("client", msg);
}

fn connect_with_retry() -> Result<std::fs::File>
{
        for attempt in 1..=10 {
                let file_res = OpenOptions::new().read(true).write(true).open(PIPE_NAME);
                if let Ok(file) = file_res {
                        if attempt > 1 {
                                log_client(&format!("Pipe connected on attempt {}", attempt));
                        }
                        return Ok(file);
                }

                let e = file_res.unwrap_err();
                let os_err = e.raw_os_error();

                match os_err {
                        // ERROR_PIPE_BUSY (231) - wait for pipe to become available
                        Some(231) => {
                                log_client(&format!("Pipe busy, waiting... (attempt {})", attempt));
                                let pipe_name_wide: Vec<u16> =
                                        PIPE_NAME.encode_utf16().chain(std::iter::once(0)).collect();
                                unsafe {
                                        let _ = WaitNamedPipeW(PCWSTR(pipe_name_wide.as_ptr()), 2000);
                                }
                        }
                        // ERROR_FILE_NOT_FOUND (2) - service might be starting
                        Some(2) => {
                                log_client(&format!("Pipe not found, retrying... (attempt {})", attempt));
                                std::thread::sleep(std::time::Duration::from_millis(200));
                        }
                        // Any other error
                        _ => {
                                log_client(&format!(
                                        "Pipe connect failed: {:?} (os_error={:?}) (attempt {})",
                                        e, os_err, attempt
                                ));
                                if attempt >= 10 {
                                        return Err(anyhow!(
                                                "Failed to connect to pipe after {} attempts: {}",
                                                attempt,
                                                e
                                        ));
                                }
                                std::thread::sleep(std::time::Duration::from_millis(200));
                        }
                }
        }

        Err(anyhow!("Failed to connect to pipe after 10 attempts"))
}

/// Send a command to the service. Thread-safe via mutex.
/// Uses a persistent connection.
pub fn send_command(cmd: ServiceCommand) -> Result<ServiceResponse>
{
        let mut conn_guard = PIPE_CONNECTION.lock().unwrap_or_else(|e| e.into_inner());

        // Connect if not connected
        if conn_guard.is_none() {
                let file = connect_with_retry().context("Failed to connect to W11BoostSvc pipe")?;
                *conn_guard = Some(file);
        }

        let file = conn_guard.as_mut().unwrap();

        let req_bytes = match serde_json::to_vec(&cmd) {
                Ok(b) => b,
                Err(e) => return Err(e.into()),
        };

        if let Err(e) = file.write_all(&req_bytes) {
                *conn_guard = None; // Force reconnect next time
                return Err(anyhow!("Failed to write to service pipe: {}", e));
        }

        if let Err(e) = file.flush() {
                *conn_guard = None;
                return Err(anyhow!("Failed to flush service pipe: {}", e));
        }

        let mut buf = [0u8; 4096];
        match file.read(&mut buf) {
                Ok(n) => {
                        if n == 0 {
                                *conn_guard = None;
                                return Err(anyhow!("Service closed connection or empty response"));
                        }
                        let resp: ServiceResponse = match serde_json::from_slice(&buf[..n]) {
                                Ok(r) => r,
                                Err(e) => return Err(anyhow!("Failed to deserialize response: {}", e)),
                        };
                        Ok(resp)
                }
                Err(e) => {
                        *conn_guard = None;
                        return Err(anyhow!("Failed to read from service pipe: {}", e));
                }
        }
}

/// Kill any zombie W11BoostSvc.exe processes
fn cleanup_zombie_services()
{
        log_client("Cleaning up zombie services...");

        // Kill legacy W11BoostSvc.exe if present
        let _ = std::process::Command::new("taskkill")
                .args(["/F", "/IM", "W11BoostSvc.exe"])
                .creation_flags(0x0800_0000) // CREATE_NO_WINDOW
                .output();

        // Kill legacy --service processes of W11Boost.exe
        let _ = std::process::Command::new("wmic")
                .args([
                        "process",
                        "where",
                        "Name='W11Boost.exe' and CommandLine like '%--service%'",
                        "call",
                        "terminate",
                ])
                .creation_flags(0x0800_0000)
                .output();

        std::thread::sleep(std::time::Duration::from_millis(500));
}

/// Find service binary - which is now THIS binary
fn find_service_binary() -> Result<std::path::PathBuf>
{
        let current_exe = std::env::current_exe()?;
        log_client(&format!("Service binary is current exe: {:?}", current_exe));
        Ok(current_exe)
}

/// Wait for service to respond to ping
fn wait_for_service_ready() -> Result<()>
{
        log_client("Waiting for service to become ready...");

        for attempt in 0..20 {
                std::thread::sleep(std::time::Duration::from_millis(250));

                match send_command(ServiceCommand::Ping) {
                        Ok(ServiceResponse::Success) => {
                                log_client(&format!("Service ready after {} attempts", attempt + 1));
                                return Ok(());
                        }
                        Ok(ServiceResponse::Error(e)) => {
                                log_client(&format!("Ping error: {}", e));
                        }
                        Err(e) => {
                                log_client(&format!("Ping attempt {} failed: {}", attempt + 1, e));
                        }
                }
        }

        Err(anyhow!("Timed out waiting for W11BoostSvc to start (5s)"))
}

pub fn ensure_service_running() -> Result<()>
{
        log_client("ensure_service_running() called");

        // Check if service is already running and responsive
        if let Ok(ServiceResponse::Success) = send_command(ServiceCommand::Ping) {
                log_client("Service already running and responsive");
                return Ok(());
        }

        log_client("Service not responsive, will start it");

        // Kill any zombie processes
        cleanup_zombie_services();

        // Find the service binary (current exe)
        let svc_path = find_service_binary()?;

        log_client("Spawning service as TrustedInstaller...");

        // Spawn with TrustedInstaller privileges
        if !crate::trusted_installer::spawn_process_as_trusted_installer(&svc_path, "--service") {
                return Err(anyhow!(
                        "Failed to spawn W11BoostSvc as TrustedInstaller (check ti_debug.log)"
                ));
        }

        wait_for_service_ready()
}
