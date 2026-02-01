use crate::cmd::Command;

#[allow(unused)]
pub struct GetCommand {
    pub value: String,
}

impl Command for GetCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", self.value);
        Ok(())
    }
}
