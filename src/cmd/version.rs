use crate::cmd::Command;

pub struct VersionCommand;

impl Command for VersionCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let version: &str = env!("CARGO_PKG_VERSION");
        println!("rustport version {}", version);
        Ok(())
    }
}
