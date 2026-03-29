use std::{
    fmt::Display,
    io::{self, Write},
    process::exit,
};

use color_print::{ceprintln, cprintln};
use rand::Rng;
use regex::Regex;
use tokio::{runtime::Handle, task::block_in_place};
use version_compare::Version;

use crate::{
    Config,
    net::{NetworkManager, health::HealthRes},
    utilities::file::{get_configuration, get_local_information},
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
    let config: Config = match get_configuration() {
        Ok(c) => c,
        Err(_) => exit(0),
    };
    config.global.ip_addr
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
                    cprintln!("<red>You are not signed in to passport!</>");
                    cprintln!("<red>Run 'passport login' to sign in</>");
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

fn select_random_digit() -> String {
    let number = rand::thread_rng().gen_range(0..=9);
    number.to_string()
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
            2 => result += String::from(select_random_digit()).as_str(),
            3 => result += String::from(select_char(String::from(special_chars))).as_str(),
            _ => {}
        }
    }
    String::from(result)
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn convert_snake_case(host_name: String) -> String {
    let string_parts = host_name.split("_");
    let mut result: String = String::new();
    let len = string_parts.clone().count();
    for (i, part) in string_parts.enumerate() {
        result += capitalize(&part).as_str();
        if i != len - 1 {
            result += " ";
        }
    }
    result
}

fn convert_camel_case(host_name: String) -> String {
    let result: String = host_name
        .chars()
        .flat_map(|c| {
            if c.is_uppercase() {
                vec![' ', c]
            } else {
                vec![c]
            }
        })
        .collect();
    capitalize(&result)
}

pub fn convert_host(host_name: String) -> String {
    let snake_re = Regex::new("^[a-z0-9]+(_[a-z0-9]+)*$").unwrap();
    let is_snake: bool = match snake_re.captures(&host_name) {
        Some(_) => true,
        None => false,
    };

    if is_snake {
        return convert_snake_case(String::from(host_name));
    }
    let camel_re = Regex::new("^[a-z]+([A-Z][a-z0-9]*)+$").unwrap();
    let is_camel: bool = match camel_re.captures(&host_name) {
        Some(_) => true,
        None => false,
    };

    if is_camel {
        return convert_camel_case(String::from(host_name));
    }
    capitalize(&host_name)
}

#[allow(unused)]
pub fn update_available() -> bool {
    let nm: NetworkManager = NetworkManager::new();
    let mut health: HealthRes = HealthRes {
        health: String::new(),
        rustport_version: String::new(),
    };
    block_in_place(|| {
        Handle::current().block_on(async {
            health = match nm.health().await {
                Ok(v) => v,
                Err(_) => HealthRes {
                    health: "ConnectionFailed".to_string(),
                    rustport_version: "N/A".to_string(),
                },
            };
        })
    });
    if health.health == "ConnectionFailed" {
        return false;
    }
    let current_version: Version = Version::from(env!("CARGO_PKG_VERSION")).unwrap();
    let latest_version: Version = Version::from(&health.rustport_version).unwrap();
    if current_version < latest_version {
        return true;
    }
    false
}

pub fn vec_contains(vec: Vec<String>, str: String) -> bool {
    for item in vec {
        if item == str {
            return true;
        }
    }
    false
}
