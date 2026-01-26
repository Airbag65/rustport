use core::fmt;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{env, fs::File, io::BufReader, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInformation {
    pub name: String,
    pub surname: String,
    pub email: String,
    pub auth_token: String,
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

impl fmt::Display for UserInformation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Name: {} {}\nAuth token: {}\nEmail: {}",
            self.name, self.surname, self.auth_token, self.email
        )
    }
}
