use anyhow::Result;

use w11boost::run_system_command;

pub fn run() -> Result<()>
{
        run_system_command("wsreset.exe", &["-i"])?;
        Ok(())
}
