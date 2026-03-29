use color_print::cprintln;

use crate::{cmd::Command, utilities::file::read_file};

pub struct HelpCommand;

impl Command for HelpCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let title = read_file("passport_title.txt")?;
        cprintln!("<rgb(33, 170, 255)>{}</>", title);
        println!("Usage: passport <command> [arguments]");
        println!("COMMANDS:");
        println!("{:<30} {}", " init", "Choose IP address to target");
        println!("{:<30} {}", " status", "Check login status");
        println!("{:<30} {}", " login", "Login to passport");
        println!("{:<30} {}", " signout", "Sign out from passport");
        println!("{:<30} {}", " register", "Sign up new user to passport");
        println!("{:<30} {}", " add", "Add a new password");
        println!(
            "{:<30} {}",
            " get [-h --host] <hostname>", "Retrieve the password for the specified hostname"
        );
        println!(
            "{:<30} {}",
            " list", "List all the hosts you have registered passwords for"
        );
        println!(
            "{:<30} {}",
            " remove [-h --host] <hostname>", "Remove the password for the specified hostname"
        );
        println!(
            "{:<30} {}",
            " generate", "Generate a pseudo-random 20 characters long password"
        );
        println!(
            "{:<30} {}",
            " edit [-h --host] <hostname>", "Edit the password for the specified hostname"
        );
        println!(
            "{:<30} {}",
            " reset_account", "Send an account-reset-request to server"
        );
        println!(
            "{:<30} {}",
            " alias <command> <alias>", "Add an alias for a command"
        );
        println!();
        println!(
            "{:<30} {}",
            " help", "Lists all possible commands and their usage"
        );
        println!(
            "{:<30} {}",
            " version", "Show the installed passport version"
        );
        println!("{:<30} {}", " view", "View aliases for all commands");
        println!("{:<30} {}", " config", "Edit passport configuration file");

        Ok(())
    }
}
