#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![deny(
        clippy::cargo,
        clippy::complexity,
        clippy::correctness,
        clippy::perf,
        clippy::style,
        clippy::suspicious
)]
#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
        clippy::too_many_lines,
        clippy::missing_errors_doc,
        clippy::multiple_crate_versions,
        clippy::single_call_fn,
        clippy::implicit_return,
        clippy::missing_docs_in_private_items,
        clippy::min_ident_chars,
        clippy::arbitrary_source_item_ordering,
        clippy::question_mark_used
)]

use w11boost::common;
use w11boost::gui;

use std::panic;

fn is_network_path() -> bool
{
        let Ok(exe_path) = std::env::current_exe() else {
                return false;
        };

        let path_str = exe_path.to_string_lossy();
        // UNC paths start with \\ (e.g., \\server\share\...)
        path_str.starts_with(r"\\")
}

fn main()
{
        let args: Vec<String> = std::env::args().collect();

        // Check for service mode first
        if args.len() > 1 && args[1] == "--service" {
                if let Err(e) = w11boost::service::run() {
                        // We can't easily log to GUI, so just log to file
                        common::log_debug("service", &format!("Service run failed: {}", e));
                }
                return;
        }

        common::log_debug("main", &format!("main() started, args: {:?}", args));

        // Block running from network drives - TrustedInstaller cannot access UNC paths
        if is_network_path() {
                common::log_debug("main", "ERROR: Running from network drive detected, exiting");
                rfd::MessageDialog::new()
			.set_title("W11Boost")
			.set_description("W11Boost cannot run from a network drive.\n\nPlease copy the files to a local drive and try again.")
			.set_level(rfd::MessageLevel::Error)
			.show();
                return;
        }

        common::log_debug("main", "Local path verified, starting GUI mode");
        // Start W11BoostSvc for privileged operations (Mandatory coupling)
        if let Err(e) = w11boost::service_client::ensure_service_running() {
                common::log_debug("main", &format!("Failed to start service: {}", e));
                eprintln!("Service Error: Failed to start background service:\n{}", e);
                // We continue? Yes, but features will fail.
        } else {
                common::log_debug("main", "Service started successfully");
        }

        // Set up panic handler to log to stderr
        panic::set_hook(Box::new(|info| {
                let location = info
                        .location()
                        .map_or_else(|| "unknown".to_string(), std::string::ToString::to_string);

                let payload = info.payload();
                let msg = if let Some(s) = payload.downcast_ref::<&str>() {
                        format!("Panic: {s}\n\nLocation: {location}")
                } else if let Some(s) = payload.downcast_ref::<String>() {
                        format!("Panic: {s}\n\nLocation: {location}")
                } else {
                        format!("Panic occurred\n\nLocation: {location}")
                };

                eprintln!("{}", msg);
                common::log_debug("main", &format!("PANIC: {}", msg));
        }));

        if let Err(e) = gui::run_gui() {
                eprintln!("W11Boost Error: run_gui() failed.\nError: {}", e);
                common::log_debug("main", &format!("GUI Error: {}", e));
        }
}
