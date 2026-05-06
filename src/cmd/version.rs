use crate::cmd::Command;

pub struct VersionCommand;

impl Command for VersionCommand {
    fn execute(&self) -> Result<(), anyhow::Error> {
        let version: &str = env!("CARGO_PKG_VERSION");
        println!("passport version {}", version);
        Ok(())
    }
}
