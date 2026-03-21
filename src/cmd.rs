use std::{env, process::exit};

use color_print::cprintln;

use crate::{
    Config,
    cmd::{
        add::AddCommand, alias::AliasCommand, edit::EditCommand, generate::GenerateCommand,
        get::GetCommand, help::HelpCommand, init::InitCommand, login::LoginCommand,
        logout::LogoutCommand, ls::LsCommand, register::RegisterCommand, rm::RemoveCommand,
        rsacc::ResetAccountCommand, status::StatusCommand, version::VersionCommand,
        view::ViewCommand,
    },
    utilities::{file::get_configuration, vec_contains},
};

pub mod add;
pub mod alias;
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
pub mod view;

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
    let config: Config = match get_configuration() {
        Ok(v) => v,
        Err(_) => {
            cprintln!("<red>Something went wrong</>");
            exit(0);
        }
    };
    if !&config.alias.is_some() {
        cprintln!("<red>Something went wrong</>");
        exit(0);
    }

    let aliases = config.alias.unwrap();

    if command_string.as_str() == "status" || vec_contains(aliases.status, command_string.clone()) {
        return Some(Box::new(StatusCommand));
    } else if command_string.as_str() == "login"
        || vec_contains(aliases.login, command_string.clone())
    {
        return Some(Box::new(LoginCommand));
    } else if command_string.as_str() == "init"
        || vec_contains(aliases.init, command_string.clone())
    {
        return Some(Box::new(InitCommand));
    } else if command_string.as_str() == "signout"
        || vec_contains(aliases.signout, command_string.clone())
    {
        return Some(Box::new(LogoutCommand));
    } else if command_string.as_str() == "register"
        || vec_contains(aliases.register, command_string.clone())
    {
        return Some(Box::new(RegisterCommand));
    } else if command_string.as_str() == "list"
        || vec_contains(aliases.list, command_string.clone())
    {
        return Some(Box::new(LsCommand));
    } else if command_string.as_str() == "get" || vec_contains(aliases.get, command_string.clone())
    {
        if argument.len() != 4 {
            eprintln!(
                "Too few arguments!\nUsage: rustport {} [-h --host] <value>",
                argument[1]
            );
            return None;
        }
        if argument[2] == "-h" || argument[2] == "--host" {
            return Some(Box::new(GetCommand {
                value: argument[3].clone(),
            }));
        }
        eprintln!(
            "Invalid flag!\nUsage: rustport {} [-h --host] <value>",
            argument[1]
        );
        return None;
    } else if command_string.as_str() == "add" || vec_contains(aliases.add, command_string.clone())
    {
        return Some(Box::new(AddCommand));
    } else if command_string.as_str() == "help"
        || vec_contains(aliases.help, command_string.clone())
    {
        return Some(Box::new(HelpCommand));
    } else if command_string.as_str() == "remove"
        || vec_contains(aliases.remove, command_string.clone())
    {
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
    } else if command_string.as_str() == "edit"
        || vec_contains(aliases.edit, command_string.clone())
    {
        if argument.len() != 4 {
            eprintln!(
                "Too few arguments!\nUsage: rustport {} [-h --host] <value>",
                argument[1]
            );
            return None;
        }
        if argument[2] == "-h" || argument[2] == "--host" {
            return Some(Box::new(EditCommand {
                value: argument[3].clone(),
            }));
        }
        eprintln!(
            "Invalid flag!\nUsage: rustport {} [-h --host] <value>",
            argument[1]
        );
        return None;
    } else if command_string.as_str() == "reset_account"
        || vec_contains(aliases.reset_account, command_string.clone())
    {
        return Some(Box::new(ResetAccountCommand));
    } else if command_string.as_str() == "generate"
        || vec_contains(aliases.generate, command_string.clone())
    {
        return Some(Box::new(GenerateCommand));
    } else if command_string.as_str() == "version"
        || vec_contains(aliases.version, command_string.clone())
    {
        return Some(Box::new(VersionCommand));
    } else if command_string.as_str() == "alias"
        || vec_contains(aliases.alias, command_string.clone())
    {
        if argument.len() != 4 {
            eprintln!(
                "Too few arguments!\nUsage: rustport {} <command> <alias>",
                argument[1]
            );
            return None;
        }
        return Some(Box::new(AliasCommand {
            command: argument[2].clone(),
            alias: argument[3].clone(),
        }));
    } else if command_string.as_str() == "view"
        || vec_contains(aliases.view, command_string.clone())
    {
        return Some(Box::new(ViewCommand));
    } else {
        eprintln!("rustport: Invalid argument");
        return None;
    }
}
