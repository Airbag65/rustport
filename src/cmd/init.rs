use color_print::cprintln;

use crate::{
    Config, cmd,
    utilities::{
        file::{get_configuration, update_config},
        read_input,
    },
};

pub struct InitCommand;

impl cmd::Command for InitCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let ip_addr = read_input("Enter IP address: ")?;
        let mut config: Config = get_configuration()?;
        config.global.ip_addr = ip_addr.clone();
        update_config(&config)?;
        cprintln!("<green>Now using:</> {}", &ip_addr);
        Ok(())
    }
}
