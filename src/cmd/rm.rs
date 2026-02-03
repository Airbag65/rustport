use crate::cmd::Command;

pub struct RemoveCommand {
    #[allow(unused)]
    pub value: String,
}

impl Command for RemoveCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
