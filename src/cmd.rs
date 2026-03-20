use std::{env, process::exit};

use color_print::cprintln;

use crate::cmd::{
    add::AddCommand, edit::EditCommand, generate::GenerateCommand, get::GetCommand,
    help::HelpCommand, init::InitCommand, login::LoginCommand, logout::LogoutCommand,
    ls::LsCommand, register::RegisterCommand, rm::RemoveCommand, rsacc::ResetAccountCommand,
    status::StatusCommand, version::VersionCommand,
};

pub mod add;
pub mod edit;
pub mod generate;
pub mod get;
pub mod help;
pub mod init;
pub mod login;
pub mod logout;
pub mod ls;
pub mod register;
pub mod rm;
pub mod rsacc;
pub mod status;
pub mod version;

pub trait Command {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>>;
}

pub fn get_command() -> Option<Box<dyn Command>> {
    let argument: Vec<String> = env::args()
        .map(|arg| arg.to_string())
        .collect::<Vec<String>>();
    if argument.len() < 2 {
        println!("Usage: rustport <command>");
        cprintln!("<yellow>Run 'rustport help' for further instructions</>");
        exit(0);
    }
    let command_string = String::from(argument[1].clone());
    match command_string.as_str() {
        "status" | "st" => return Some(Box::new(StatusCommand)),
        "login" | "lo" => return Some(Box::new(LoginCommand)),
        "init" => return Some(Box::new(InitCommand)),
        "signout" | "so" => return Some(Box::new(LogoutCommand)),
        "register" | "reg" => return Some(Box::new(RegisterCommand)),
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
        "help" | "h" => return Some(Box::new(HelpCommand)),
        "rm" | "remove" => {
            if argument.len() != 4 {
                eprintln!(
                    "Too few arguments!\nUsage: rustport {} [-h --host] <value>",
                    argument[1]
                );
                return None;
            }
            if argument[2] == "-h" || argument[2] == "--host" {
                return Some(Box::new(RemoveCommand {
                    value: argument[3].clone(),
                }));
            }
            eprintln!(
                "Invalid flag!\nUsage: rustport {} [-h --host] <value>",
                argument[1]
            );
            return None;
        }
        "edit" => {
            if argument.len() != 4 {
                eprintln!("Too few arguments!\nUsage: rustport edit [-h --host] <value>");
                return None;
            }
            if argument[2] == "-h" || argument[2] == "--host" {
                return Some(Box::new(EditCommand {
                    value: argument[3].clone(),
                }));
            }
            eprintln!("Invalid flag!\nUsage: rustport edit [-h --host] <value>");
            return None;
        }
        "rsacc" | "reset_account" => return Some(Box::new(ResetAccountCommand)),
        "generate" | "gen" => return Some(Box::new(GenerateCommand)),
        "version" | "v" => return Some(Box::new(VersionCommand)),
        _ => {
            eprintln!("rustport: Invalid argument");
            return None;
        }
    };
}
