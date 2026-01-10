// Software Installation via Winget

use super::{Tweak, TweakEffect};
use crate::gui::shared_state::WorkerContext;
use anyhow::Result;
use std::os::windows::process::CommandExt;
use std::process::Command;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

macro_rules! winget_tweak {
        ($id:expr, $name:expr, $desc:expr, $package_id:expr) => {
                winget_tweak!($id, $name, $desc, $package_id, &[])
        };
        ($id:expr, $name:expr, $desc:expr, $package_id:expr, $args:expr) => {
                crate::tweak! {
                    id: $id,
                    category: "software",
                    name: $name,
                    description: $desc,
                    effect: TweakEffect::Immediate,
                    enabled_ops: &[],
                    disabled_ops: None,
                    custom_apply: Some(|ctx| install_winget($package_id, $args, ctx))
                }
        };
}

pub static SOFTWARE_TWEAKS: &[Tweak] = &[
        winget_tweak!(
                "install_icaros",
                "Install Icaros",
                "Installs Icaros (shell extensions for the thumbnail generation of videos, images, and other file types).",
                "Xanashi.Icaros",
                &["--scope", "machine"]
        ),
        winget_tweak!(
                "install_startallback",
                "Install StartAllBack",
                "Installs StartAllBack (Windows 11 taskbar and start menu customization).",
                "StartIsBack.StartAllBack",
                &["--scope", "machine"]
        ),
        winget_tweak!(
                "install_fortfirewall",
                "Install Fort Firewall",
                "Installs Fort Firewall (the most comprehensive open-source firewall for Windows).",
                "NodirTemirkhodjaev.FortFirewall"
        ),
        winget_tweak!(
                "install_7zip",
                "Install 7-Zip",
                "Installs 7-Zip (the best open-source file archiver).",
                "7zip.7zip"
        ),
];

#[allow(clippy::unnecessary_wraps)]
fn install_winget(id: &str, extra_args: &[&str], ctx: &Arc<WorkerContext>) -> Result<()>
{
        ctx.post_status(&format!("Installing {id} via winget..."));

        let args = build_winget_args(id, extra_args);
        let max_retries = 5;

        for attempt in 1..=max_retries + 1 {
                if attempt > max_retries {
                        // Fallthrough if loop finishes without break
                        ctx.post_status(&format!("  Failed to install {id} after {max_retries} retries"));
                        break;
                }

                if let Some(should_retry) = try_winget_install(id, &args, attempt, ctx) {
                        if !should_retry {
                                break;
                        }
                } else {
                        // Error occurred executing command
                        break;
                }
        }

        ctx.report_progress();
        Ok(())
}

fn build_winget_args<'a>(id: &'a str, extra_args: &'a [&'a str]) -> Vec<&'a str>
{
        let mut args = vec![
                "install",
                "--id",
                id,
                "--exact",
                "--silent",
                "--accept-source-agreements",
                "--accept-package-agreements",
                "--disable-interactivity",
        ];
        args.extend_from_slice(extra_args);
        args
}

fn try_winget_install(id: &str, args: &[&str], attempt: u32, ctx: &Arc<WorkerContext>) -> Option<bool>
{
        let output_res = Command::new("winget.exe")
                .args(args)
                .creation_flags(crate::common::CREATE_NO_WINDOW)
                .output();

        match output_res {
                Ok(output) => {
                        let code = output.status.code().unwrap_or(1);

                        if output.status.success() {
                                ctx.post_status(&format!("  Successfully installed {id}"));
                                return Some(false); // Done, don't retry
                        }

                        let stdout = String::from_utf8_lossy(&output.stdout);
                        // Some installers return 0 even if "already installed", but some might error.
                        // We check the text output as a fallback.
                        if stdout.contains("already installed") {
                                ctx.post_status(&format!("  {id} is already installed"));
                                return Some(false);
                        }

                        // Check for concurrency issues
                        // 1618 = ERROR_INSTALL_ALREADY_RUNNING
                        if code == 1618 {
                                let wait_secs = 2u64.pow(attempt - 1);
                                ctx.post_status(&format!(
                                        "  Installation locked (another install running). Retrying in {wait_secs}s..."
                                ));
                                thread::sleep(Duration::from_secs(wait_secs));
                                return Some(true); // Retry
                        }

                        ctx.post_status(&format!("  Failed to install {id} (Exit code: {code})"));
                        Some(false)
                }
                Err(e) => {
                        ctx.post_status(&format!("  Failed to execute winget: {e}"));
                        None
                }
        }
}
