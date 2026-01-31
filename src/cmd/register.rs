use color_print::cprintln;
use scanpw::scanpw;

use crate::{cmd::Command, utilities::read_input};

pub struct RegisterCommand;

impl Command for RegisterCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Sign up new user");
        println!("----------------");
        let email: String = read_input("Email: ")?;
        let name: String = read_input("First name: ")?;
        let surname: String = read_input("Surname")?;
        let mut password: String;
        let mut confirm_password: String;
        loop {
            password = scanpw!("Password: ");
            println!();
            confirm_password = scanpw!("Confirm password: ");
            println!();
            if password == confirm_password {
                break;
            }
            cprintln!("<red>Passwords don't match</>");
        }
        Ok(())
    }
}
