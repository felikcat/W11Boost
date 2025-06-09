use std::{error::Error, process::Command};

pub fn run() -> Result<(), Box<dyn Error>> {
        let mut child = Command::new("wsreset.exe").args(&["-i"]).spawn()?;
        child.wait()?;

        Ok(())
}
