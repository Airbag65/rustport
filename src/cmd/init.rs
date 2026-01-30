use crate::cmd;

pub struct InitCommand;

impl cmd::Command for InitCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        // todo!(
        //     "Get IP\nSet IP in config file or shell variable\nFor other commands - ensure IP exists before doing other stuff"
        // );
        Ok(())
    }
}
