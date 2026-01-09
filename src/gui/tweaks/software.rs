// Software Installation via Winget

use super::{Tweak, TweakEffect};
use crate::gui::shared_state::WorkerContext;
use anyhow::Result;
use std::os::windows::process::CommandExt;
use std::process::Command;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub static SOFTWARE_TWEAKS: &[Tweak] = &[
        crate::tweak! {
                id: "install_icaros",
                category: "software",
                name: "Install Icaros",
                description: "Installs Icaros (shell extensions for video thumbnails).",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| install_winget("Xanashi.Icaros", &["--scope", "machine"], ctx))
        },
        crate::tweak! {
                id: "install_startallback",
                category: "software",
                name: "Install StartAllBack",
                description: "Installs StartAllBack (Windows 11 taskbar and start menu customization).",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| install_winget("StartIsBack.StartAllBack", &["--scope", "machine"], ctx))
        },
        crate::tweak! {
                id: "install_fortfirewall",
                category: "software",
                name: "Install Fort Firewall",
                description: "Installs Fort Firewall (simple firewall for Windows).",
                effect: TweakEffect::Immediate,
                enabled_ops: &[],
                disabled_ops: None,
                custom_apply: Some(|ctx| install_winget("NodirTemirkhodjaev.FortFirewall", &[], ctx))
        },
];

#[allow(clippy::unnecessary_wraps)]
#[allow(clippy::unnecessary_wraps)]
fn install_winget(id: &str, extra_args: &[&str], ctx: &Arc<WorkerContext>) -> Result<()>
{
        ctx.post_status(&format!("Installing {id} via winget..."));

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

        let max_retries = 5;
        let mut attempt = 0;

        loop {
                attempt += 1;

                // Use std::process::Command directly to get exit code
                let output_res = Command::new("winget.exe")
                        .args(&args)
                        .creation_flags(crate::common::CREATE_NO_WINDOW)
                        .output();

                match output_res {
                        Ok(output) => {
                                let code = output.status.code().unwrap_or(1);

                                if output.status.success() {
                                        ctx.post_status(&format!("  Successfully installed {id}"));
                                        break;
                                } else {
                                        let stdout = String::from_utf8_lossy(&output.stdout);
                                        // Some installers return 0 even if "already installed", but some might error.
                                        // We check the text output as a fallback.
                                        if stdout.contains("already installed") {
                                                ctx.post_status(&format!("  {id} is already installed"));
                                                break;
                                        }

                                        // Check for concurrency issues
                                        // 1618 = ERROR_INSTALL_ALREADY_RUNNING
                                        if code == 1618 && attempt <= max_retries {
                                                let wait_secs = 2u64.pow(attempt - 1);
                                                ctx.post_status(&format!("  Installation locked (another install running). Retrying in {wait_secs}s..."));
                                                thread::sleep(Duration::from_secs(wait_secs));
                                                continue;
                                        }

                                        ctx.post_status(&format!("  Failed to install {id} (Exit code: {code})"));
                                        break;
                                }
                        }
                        Err(e) => {
                                ctx.post_status(&format!("  Failed to execute winget: {e}"));
                                break;
                        }
                }
        }

        ctx.report_progress();
        Ok(())
}
