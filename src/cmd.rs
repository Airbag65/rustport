use std::{env, process::exit};

use crate::cmd::{
    add::AddCommand, get::GetCommand, help::HelpCommand, init::InitCommand, login::LoginCommand,
    logout::LogoutCommand, ls::LsCommand, register::RegisterCommand, rm::RemoveCommand,
    status::StatusCommand,
};

pub mod add;
pub mod get;
pub mod help;
pub mod init;
pub mod login;
pub mod logout;
pub mod ls;
pub mod register;
pub mod rm;
pub mod status;

pub trait Command {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>>;
}

pub fn get_command() -> Option<Box<dyn Command>> {
    let argument: Vec<String> = env::args()
        .map(|arg| arg.to_string())
        .collect::<Vec<String>>();
    if argument.len() < 2 {
        println!("Usage: rustport <command>");
        exit(0);
    }
    let command_string = String::from(argument[1].clone());
    match command_string.as_str() {
        "status" => return Some(Box::new(StatusCommand)),
        "st" => return Some(Box::new(StatusCommand)),
        "login" => return Some(Box::new(LoginCommand)),
        "init" => return Some(Box::new(InitCommand)),
        "signout" => return Some(Box::new(LogoutCommand)),
        "register" => return Some(Box::new(RegisterCommand)),
        "signup" => return Some(Box::new(RegisterCommand)),
        "ls" | "list" => return Some(Box::new(LsCommand)),
        "get" => {
            if argument.len() != 4 {
                eprintln!("Too few arguments!\nUsage: rustport get [-h --host] <value>");
                return None;
            }
            if argument[2] == "-h" || argument[2] == "--host" {
                return Some(Box::new(GetCommand {
                    value: argument[3].clone(),
                }));
            }
            eprintln!("Invalid flag!\nUsage: rustport get [-h --host] <value>");
            return None;
        }
        "add" => return Some(Box::new(AddCommand)),
        "help" => return Some(Box::new(HelpCommand)),
        "rm" | "remove" => {
            if argument.len() != 4 {
                eprintln!("Too few arguments!\nUsage: rustport rm [-h --host] <value>");
                return None;
            }
            if argument[2] == "-h" || argument[2] == "--host" {
                return Some(Box::new(RemoveCommand {
                    value: argument[3].clone(),
                }));
            }
            eprintln!("Invalid flag!\nUsage: rustport rm [-h --host] <value>");
            return None;
        }
        _ => {
            eprintln!("rustport: Invalid argument");
            return None;
        }
    };
}
