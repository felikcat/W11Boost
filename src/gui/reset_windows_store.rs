use anyhow::Result;

use crate::common::run_system_command;

/// The Windows Store reset executable
pub const WSRESET_EXECUTABLE: &str = "wsreset.exe";

/// Command line argument for installing/resetting the store
pub const WSRESET_INSTALL_ARG: &str = "-i";

pub fn run() -> Result<()>
{
        run_system_command(WSRESET_EXECUTABLE, &[WSRESET_INSTALL_ARG])?;
        Ok(())
}
