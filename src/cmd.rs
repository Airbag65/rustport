use std::{env, process::exit};

use crate::cmd::status::StatusCommand;

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
        _ => return None,
    };
}
