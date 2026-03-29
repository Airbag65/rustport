use core::fmt;
use std::process::exit;

use serde::{Deserialize, Serialize};

use crate::utilities::{print_boxed, update_available};

mod cmd;
mod net;
mod utilities;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInformation {
    pub name: String,
    pub surname: String,
    pub email: String,
    pub auth_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub global: Global,
    pub alias: Option<Alias>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Global {
    pub source_path: String,
    pub ip_addr: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Alias {
    pub list: Vec<String>,
    pub add: Vec<String>,
    pub edit: Vec<String>,
    pub generate: Vec<String>,
    pub get: Vec<String>,
    pub help: Vec<String>,
    pub version: Vec<String>,
    pub init: Vec<String>,
    pub login: Vec<String>,
    pub signout: Vec<String>,
    pub register: Vec<String>,
    pub remove: Vec<String>,
    pub reset_account: Vec<String>,
    pub status: Vec<String>,
    pub alias: Vec<String>,
    pub view: Vec<String>,
    pub config: Vec<String>,
}

struct VecWrapper(Vec<String>);

impl fmt::Display for VecWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.join(", "))
    }
}

impl fmt::Display for UserInformation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Name: {} {}\nAuth token: {}\nEmail: {}",
            self.name, self.surname, self.auth_token, self.email
        )
    }
}

impl fmt::Display for Alias {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:<15} {}\n{:<15} {}\n{:<15} {}\n{:<15} {}\n{:<15} {}\n{:<15} {}\n{:<15} {}\n{:<15} {}\n{:<15} {}\n{:<15} {}\n{:<15} {}\n{:<15} {}\n{:<15} {}\n{:<15} {}\n{:<15} {}\n{:<15} {}\n{:<15} {}\n{:<15} {}",
            "COMMAND",
            "ALIAS",
            "list:",
            VecWrapper(self.list.clone()),
            "add:",
            VecWrapper(self.add.clone()),
            "edit:",
            VecWrapper(self.edit.clone()),
            "generate:",
            VecWrapper(self.generate.clone()),
            "get:",
            VecWrapper(self.get.clone()),
            "help:",
            VecWrapper(self.help.clone()),
            "version:",
            VecWrapper(self.version.clone()),
            "init:",
            VecWrapper(self.init.clone()),
            "login:",
            VecWrapper(self.login.clone()),
            "signout:",
            VecWrapper(self.signout.clone()),
            "register:",
            VecWrapper(self.register.clone()),
            "remove:",
            VecWrapper(self.remove.clone()),
            "reset_account:",
            VecWrapper(self.reset_account.clone()),
            "status:",
            VecWrapper(self.status.clone()),
            "alias:",
            VecWrapper(self.alias.clone()),
            "view:",
            VecWrapper(self.view.clone()),
            "config:",
            VecWrapper(self.config.clone())
        )
    }
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    if update_available() {
        print_boxed("   NOTICE!   ");
        println!("There is a new passport version available!");
        println!("Run 'passport-update' to upgrade to the newest version");
        println!("------------------------------------------------------");
    }
    let command = match cmd::get_command() {
        Some(cmd) => cmd,
        None => {
            exit(0);
        }
    };
    command.execute()?;
    Ok(())
}
