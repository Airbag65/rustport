use std::{env, process::exit};

use color_print::cprintln;

use crate::cmd::Command;

pub struct ConfigCommand;

impl Command for ConfigCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut path = match env::home_dir() {
            Some(p) => p,
            None => {
                cprintln!("<red>Something went wrong</>");
                exit(0);
            }
        };
        path.push(".passport");
        path.push("config.toml");
        edit::edit_file(path)?;
        Ok(())
    }
}
