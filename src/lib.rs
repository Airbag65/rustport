use core::fmt;
use std::{env, fs::File, io::BufReader, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::net::NetworkManager;

mod net;
mod parse;

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

    let file: File = File::open(full_path)?;
    let reader = BufReader::new(file);
    let user: UserInformation = serde_json::from_reader(reader)?;
    Ok(user)
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    parse::say_hello();
    let local_user = match get_local_information() {
        Ok(person) => person,
        Err(e) => return Err(e),
    };
    println!("{}", local_user);
    #[allow(unused_variables)]
    let nm: NetworkManager = NetworkManager::new();
    let token_valid = nm.validate_token(&local_user.auth_token).await?;
    if token_valid {
        println!("Everything good!");
    } else {
        println!("Nothing good!");
    }
    // for arg in env::args() {
    //     println!("{arg}");
    // }
    Ok(())
}
