use std::{process::exit, str::FromStr};

use color_print::cprintln;
use toml::Table;

use crate::{
    cmd::Command,
    utilities::file::{get_configuration, update_config},
};

#[allow(unused)]
pub struct AliasCommand {
    pub command: String,
    pub alias: String,
}

impl Command for AliasCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut config_aliases = get_configuration()?;
        let mut alias_str = toml::to_string(
            &config_aliases
                .alias
                .as_ref()
                .expect("Could not convert toml aliases to string"),
        )?;
        let possible_commands: Vec<&str> = alias_str
            .as_str()
            .lines()
            .map(|command| command.split('=').next().unwrap().trim())
            .collect();

        if !possible_commands
            .iter()
            .any(|host| self.command.contains(host))
        {
            cprintln!("<red>Command '{}' not found</>", self.command);
            exit(0);
        }
        let mut alias_table = Table::try_from(
            &config_aliases
                .alias
                .as_ref()
                .expect("Could not convert toml aliases to string"),
        )?;

        if alias_table[&self.command].is_array() {
            let new_alias: toml::Value =
                match toml::Value::from_str(format!("\"{}\"", self.alias).as_str()) {
                    Ok(v) => v,
                    Err(e) => {
                        println!("Something went wrong: {}", e);
                        exit(0);
                    }
                };
            alias_table[&self.command]
                .as_array()
                .unwrap()
                .iter()
                .for_each(|item| {
                    if item.is_str() {
                        if item.as_str() == Some(&self.alias.as_str()) {
                            println!(
                                "Alias '{}' for '{}' already exists",
                                self.alias, self.command
                            );
                            exit(0);
                        }
                    }
                });
            alias_table[&self.command]
                .as_array_mut()
                .unwrap()
                .push(new_alias);
        }
        alias_str = toml::to_string(&alias_table)?;
        config_aliases.alias = Some(toml::from_str(&alias_str)?);
        update_config(&config_aliases)?;
        println!("Added alias '{}' for '{}'", self.alias, self.command);
        Ok(())
    }
}
