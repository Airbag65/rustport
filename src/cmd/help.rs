use color_print::cprintln;

use crate::{cmd::Command, utilities::file::read_file};

pub struct HelpCommand;

impl Command for HelpCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let title = read_file("rustport_title.txt")?;
        cprintln!("<rgb(211, 69, 22)>{}</>", title);
        println!("Usage: rustport <command> [flag] [<value>]");
        println!("COMMANDS:");
        println!("{:<30} {}", " init", "Choose IP address to target");
        cprintln!(
            "{:<30} {} <i><c>#alias: st</></>",
            " status",
            "Check login status"
        );
        println!("{:<30} {}", " login", "Login to rustport");
        cprintln!(
            "{:<30} {} <i><c>#alias: so</></>",
            " signout",
            "Sign out from rustport"
        );
        println!("{:<30} {}", " register", "Sign up new user to rustport");
        println!("{:<30} {}", " add", "Add a new password");
        println!(
            "{:<30} {}",
            " get [-h --host] <hostname>", "Retrieve the password for the specified hostname"
        );
        cprintln!(
            "{:<30} {} <i><c>#alias: ls</></>",
            " list",
            "List all the hosts you have registered passwords for"
        );
        cprintln!(
            "{:<30} {} <i><c>#alias: rm</></>",
            " remove [-h --host] <hostname>",
            "Remove the password for the specified hostname"
        );
        cprintln!(
            "{:<30} {} <i><c>#alias: gen</></>",
            " generate",
            "Generate a pseudo-random 20 characters long password"
        );
        cprintln!(
            "{:<30} {} <i><c>#alias: h</></>",
            " help",
            "Lists all possible commands and their usage"
        );

        Ok(())
    }
}
