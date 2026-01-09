use super::shared_state::WorkerContext;
use super::tweaks::{Tweak, revert_tweak};
use anyhow::Result;
use std::sync::Arc;

pub const OP_COUNT: u32 = 0; // Calculated dynamically now

/// Run the removal process
pub fn run_revert(ctx: &Arc<WorkerContext>, tweaks: Vec<&'static Tweak>) -> Result<()>
{
        ctx.post_status("Starting removal process...");

        for tweak in tweaks {
                let msg = format!("Reverting: {}", tweak.name);
                ctx.post_status(&msg);

                if let Err(e) = revert_tweak(tweak, ctx) {
                        ctx.post_status(&format!("Failed to revert {}: {}", tweak.name, e));
                        // We continue even if one fails, best effort
                }
                ctx.report_progress();
        }

        ctx.post_status("Removal process completed.");
        Ok(())
}
