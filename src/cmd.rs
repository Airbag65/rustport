use std::{env, process::exit};

use crate::cmd::{
    init::InitCommand, login::LoginCommand, logout::LogoutCommand, ls::LsCommand,
    register::RegisterCommand, status::StatusCommand,
};

pub mod init;
pub mod login;
pub mod logout;
pub mod ls;
pub mod register;
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
        "ls" => return Some(Box::new(LsCommand)),
        "list" => return Some(Box::new(LsCommand)),
        _ => return None,
    };
}
