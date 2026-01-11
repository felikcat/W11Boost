use crate::ipc::{ServiceCommand, ServiceResponse};
use anyhow::{Context, Result, anyhow};
use std::fs::OpenOptions;
use std::io::{Read, Write};
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
        for attempt in 1..=100 {
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
                                // log_client(&format!("Pipe busy, waiting... (attempt {})", attempt));
                                let pipe_name_wide: Vec<u16> =
                                        PIPE_NAME.encode_utf16().chain(std::iter::once(0)).collect();
                                unsafe {
                                        // Wait up to 10ms for an instance
                                        let _ = WaitNamedPipeW(PCWSTR(pipe_name_wide.as_ptr()), 10);
                                }
                        }
                        // ERROR_FILE_NOT_FOUND (2) - service might be starting
                        Some(2) => {
                                // Very aggressive polling for fast startup
                                std::thread::sleep(std::time::Duration::from_millis(10));
                        }
                        // Any other error
                        _ => {
                                log_client(&format!(
                                        "Pipe connect failed: {:?} (os_error={:?}) (attempt {})",
                                        e, os_err, attempt
                                ));
                                if attempt >= 100 {
                                        return Err(anyhow!(
                                                "Failed to connect to pipe after {} attempts: {}",
                                                attempt,
                                                e
                                        ));
                                }
                                std::thread::sleep(std::time::Duration::from_millis(10));
                        }
                }
        }

        Err(anyhow!("Failed to connect to pipe after 100 attempts"))
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

/// Kill any zombie W11Boost service processes
fn cleanup_zombie_services()
{
        log_client("Cleaning up zombie services...");

        // Try to stop the service politely (by crashing it via command)
        match send_command(ServiceCommand::Stop) {
                Ok(_) => {
                        log_client("Sent Stop command to service.");
                        // Give it a moment to crash/exit
                        std::thread::sleep(std::time::Duration::from_millis(100));
                }
                Err(e) => {
                        log_client(&format!(
                                "Could not send Stop command (service likely not running): {}",
                                e
                        ));
                }
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
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

        // 500 attempts * 10ms = ~5 seconds total timeout
        for attempt in 0..500 {
                // Try to ping immediately, don't sleep first
                match send_command(ServiceCommand::Ping) {
                        Ok(ServiceResponse::Success) => {
                                if attempt > 0 {
                                        log_client(&format!("Service ready after {} attempts", attempt + 1));
                                }
                                return Ok(());
                        }
                        Ok(ServiceResponse::Error(_)) => {
                                // Service is up but returned error? weird, but keep retrying just in case
                                // or maybe log it
                                // log_client(&format!("Ping error: {}", e));
                        }
                        Err(_) => {
                                // connection failed, service probably not up yet
                        }
                }

                // Sleep briefly before next attempt
                std::thread::sleep(std::time::Duration::from_millis(10));
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
