use color_print::cprintln;

use crate::{cmd::Command, utilities::file::read_file};

pub struct HelpCommand;

impl Command for HelpCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let title = read_file("rustport_title.txt")?;
        cprintln!("<rgb(211, 69, 22)>{}</>", title);
        println!("Usage: rustport <command> [help] [<value>]");
        println!("COMMANDS:");
        println!("{:<30} {}", " init", "Choose IP address to target");
        println!("{:<30} {}", " status", "Check login status");
        println!("{:<30} {}", " st", "Shorthand for 'status'");
        println!("{:<30} {}", " login", "Login to rustport");
        println!("{:<30} {}", " signout", "Sign out from rustport");
        println!("{:<30} {}", " register", "Sign up new user to rustport");
        println!("{:<30} {}", " add", "Add a new password");
        println!(
            "{:<30} {}",
            " get [-h --host] <hostname>", "Retrieve the password for the specified hostname"
        );
        println!(
            "{:<30} {}",
            " list", "List all the hosts you have registered passwords for"
        );
        println!("{:<30} {}", " ls", "Shorthand for 'list'");
        println!(
            "{:<30} {}",
            " remove [-h --host] <hostname>", "Remove the password for the specified hostname"
        );
        println!(
            "{:<30} {}",
            " rm [-h --host] <hostname>", "Shorthand for 'remove'"
        );
        println!(
            "{:<30} {}",
            " help", "Lists all possible commands and their usage"
        );
        println!("{:<30} {}", " h", "Shorthand for 'help'");

        Ok(())
    }
}
