use anyhow::Result;

use crate::common::{run_system_command};

pub fn run() -> Result<()>
{
        run_system_command("wsreset.exe", &["-i"])?;
        Ok(())
}
