use crate::cmd::Command;

#[allow(unused)]
pub struct GenerateCommand;

impl Command for GenerateCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
