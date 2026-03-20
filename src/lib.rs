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

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub global: Global,
}

#[derive(Serialize, Deserialize)]
pub struct Global {
    pub source_path: String,
    pub ip_addr: String,
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

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    if update_available() {
        print_boxed("   NOTICE!   ");
        println!("There is a new rustport version available!");
        println!("Run 'rustport-update' to upgrade to the newest version");
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
