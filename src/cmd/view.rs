use std::process::exit;

use color_print::cprintln;

use crate::{cmd::Command, utilities::file::get_configuration};

pub struct ViewCommand;

impl Command for ViewCommand {
    fn execute(&self) -> Result<(), anyhow::Error> {
        let config = get_configuration()?;
        if !config.alias.is_some() {
            cprintln!("<red>Something went wrong</>");
            exit(0);
        }
        println!("{}", config.alias.unwrap());
        Ok(())
    }
}
