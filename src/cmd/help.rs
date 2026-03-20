use color_print::cprintln;

use crate::{cmd::Command, utilities::file::read_file};

pub struct HelpCommand;

impl Command for HelpCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let title = read_file("rustport_title.txt")?;
        cprintln!("<rgb(211, 69, 22)>{}</>", title);
        println!("Usage: rustport <command> [arguments]");
        println!("COMMANDS:");
        println!("{:<30} {}", " init", "Choose IP address to target");
        cprintln!(
            "{:<30} {} <i><c>#alias: st</></>",
            " status",
            "Check login status"
        );
        cprintln!(
            "{:<30} {} <i><c>#alias: lo</></>",
            " login",
            "Login to rustport"
        );
        cprintln!(
            "{:<30} {} <i><c>#alias: so</></>",
            " signout",
            "Sign out from rustport"
        );
        cprintln!(
            "{:<30} {} <i><c>#alias: reg</></>",
            " register",
            "Sign up new user to rustport"
        );
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
        println!(
            "{:<30} {}",
            " edit [-h --host] <hostname>", "Edit the password for the specified hostname"
        );
        cprintln!(
            "{:<30} {} <i><c>#alias: rsacc</></>",
            " reset_account",
            "Send an account-reset-request to server"
        );
        println!();
        cprintln!(
            "{:<30} {} <i><c>#alias: h</></>",
            " help",
            "Lists all possible commands and their usage"
        );
        cprintln!(
            "{:<30} {} <i><c>#alias: v</></>",
            " version",
            "Show the installed rustport version"
        );

        Ok(())
    }
}
