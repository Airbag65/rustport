use core::fmt;
use std::process::exit;

use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use crate::net::NetworkManager;

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
    let command = match cmd::get_command() {
        Some(cmd) => cmd,
        None => {
            exit(0);
        }
    };
    command.execute()?;
    Ok(())
}
