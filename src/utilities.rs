use std::{
    fmt::Display,
    io::{self, Write},
    process::exit,
};

use color_print::{ceprintln, cprintln};
use tokio::{runtime::Handle, task::block_in_place};

use crate::{
    net::NetworkManager,
    utilities::file::{get_local_information, read_file},
};

pub mod file;

pub fn read_input(prompt: impl Display) -> Result<String, Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    print!("{prompt}");
    let _ = std::io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer)?;
    buffer = buffer.trim_ascii_end().to_owned();
    Ok(buffer)
}

pub fn get_ip() -> String {
    let ip: String = match read_file("PASSPORT_IP") {
        Ok(ip) => ip,
        Err(_) => String::from("localhost"),
    };
    ip
}

pub fn ensure_auth() -> String {
    block_in_place(move || {
        Handle::current().block_on(async move {
            let nm: NetworkManager = NetworkManager::new();
            let token: String = match get_local_information() {
                Ok(auth) => auth.auth_token,
                Err(_) => {
                    ceprintln!("<red>No local auth token found</>");
                    exit(0)
                }
            };
            let _ = match nm.validate_token(&token).await {
                Ok(_) => return token,
                Err(_) => {
                    cprintln!("<red>You are not signed in to RUSTPORT!</>");
                    cprintln!("<red>Run 'rustport login' to sign in</>");
                    exit(0);
                }
            };
        })
    })
}
