use core::fmt;
use std::{
    env,
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
    process::exit,
};

use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use crate::net::NetworkManager;

mod cmd;
mod net;

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

pub fn get_local_information() -> Result<UserInformation, Box<dyn std::error::Error>> {
    let home_dir = match env::home_dir() {
        Some(path) => path,
        None => PathBuf::new(),
    };

    let home_str: &str = home_dir.to_str().unwrap();
    let full_path = String::from(home_str) + "/.passport/authentication.json";
    let file: File = match File::open(&full_path) {
        Ok(f) => f,
        Err(_) => {
            fs::create_dir(PathBuf::from(String::from(home_str) + "/.passport"))?;
            let f = fs::File::create(full_path)?;
            f
        }
    };
    let reader = BufReader::new(file);
    let user: UserInformation = serde_json::from_reader(reader)?;
    Ok(user)
}

pub fn save_pem_string(pem_string: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut path: PathBuf = env::home_dir().ok_or("Could not find home directory")?;
    path.push(".passport");
    fs::create_dir_all(&path)?;
    path.push("publicKey.pem");
    fs::write(path, pem_string)?;
    Ok(())
}
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let command = match cmd::get_command() {
        Some(cmd) => cmd,
        None => {
            println!("rustport: Invalid command");
            exit(0);
        }
    };
    command.execute()?;
    Ok(())
}
