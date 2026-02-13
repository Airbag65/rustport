use std::{
    fmt::Display,
    io::{self, Write},
    process::exit,
};

use color_print::{ceprintln, cprintln};
use rand::Rng;
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

pub fn print_boxed(content: &str) {
    print!("+");
    print!(
        "{}",
        std::iter::repeat("-")
            .take(content.len())
            .collect::<String>()
    );
    println!("+");
    print!("|");
    print!("{}", content);
    println!("|");
    print!("+");
    print!(
        "{}",
        std::iter::repeat("-")
            .take(content.len())
            .collect::<String>()
    );
    println!("+");
}

pub fn confirmation_prompt(prompt: impl Display, default_yes: bool) -> bool {
    print!("{prompt}");
    let mut choice: String;
    if default_yes {
        choice = read_input(" [Y/n] ").unwrap().to_owned();
    } else {
        choice = read_input(" [y/N] ").unwrap().to_owned();
    }
    choice = choice.to_lowercase();
    if choice == "y".to_owned() {
        true
    } else if choice != "n".to_owned() && default_yes {
        true
    } else {
        false
    }
}

fn select_char(string: String) -> char {
    let index = rand::thread_rng().gen_range(0..string.len());
    match string.chars().nth(index) {
        Some(v) => v,
        None => {
            ceprintln!("<red>Something went wrong</>");
            exit(0);
        }
    }
}

pub fn generate_password() -> String {
    let len = 20;
    let alpha = "abcdefghijklmnopqrstuvwxyz";
    #[allow(unused)]
    let nums = "0123456789";
    let special_chars = "?-=+@$&";
    let mut result = "".to_owned();
    loop {
        if result.len() == len {
            break;
        }
        let num = rand::thread_rng().gen_range(0..=3);
        match num {
            0 => result += String::from(select_char(String::from(alpha))).as_str(),
            1 => result += String::from(select_char(String::from(alpha.to_uppercase()))).as_str(),
            2 => result += String::from(select_char(num.to_string())).as_str(),
            3 => result += String::from(select_char(String::from(special_chars))).as_str(),
            _ => {}
        }
    }
    String::from(result)
}
