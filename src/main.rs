use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
struct UserInformation {
    name: String,
    surname: String,
    email: String,
    auth_token: String,
}

fn get_local_information() -> Result<UserInformation, Box<dyn std::error::Error>> {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let local_user = match get_local_information() {
        Ok(person) => person,
        Err(e) => return Err(e),
    };
    println!(
        "AuthToken: {}\nName: {} {}\nEmail: {}",
        local_user.auth_token, local_user.name, local_user.surname, local_user.email
    );
    Ok(())
}
