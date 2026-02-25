use core::time;
use std::{io::Write, thread::sleep};

use color_print::cprintln;
use scanpw::scanpw;
use tokio::{runtime::Handle, task::block_in_place};

use crate::{
    cmd::Command,
    net::NetworkManager,
    utilities::{confirmation_prompt, generate_password, print_boxed, read_input},
};

pub struct AddCommand;

impl Command for AddCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Clear the terminal window ANSI escape code
        print!("\x1B[2J\x1B[1;1H");
        let host_name: String = read_input("Enter hostname: ")?;
        let gen_password: bool = confirmation_prompt(
            "Would you like to generate a password instead of typing?",
            false,
        );
        let mut password: String;
        let mut confirm_password: String;
        if !gen_password {
            loop {
                password = scanpw!("Password: ");
                confirm_password = scanpw!("Confirm password: ");
                if password == confirm_password {
                    break;
                }
                cprintln!("<red>Passwords don't match!</>");
            }
        } else {
            password = generate_password();
            print!("Password: ");
            let _ = std::io::stdout().flush().unwrap();
            for _ in 0..password.len() {
                print!("*");
                let _ = std::io::stdout().flush().unwrap();
                sleep(time::Duration::from_millis(20));
            }
            println!();
            print!("Confirm password: ");
            let _ = std::io::stdout().flush().unwrap();
            for _ in 0..password.len() {
                print!("*");
                let _ = std::io::stdout().flush().unwrap();
                sleep(time::Duration::from_millis(20));
            }
            println!();
        }
        if confirmation_prompt("Would you like to display the password?", false) {
            print_boxed(&password);
        }
        block_in_place(move || {
            Handle::current().block_on(async move {
                println!("\nA new password is about to be saved!");
                println!("------------------------------------");
                println!("Host: {}", &host_name);
                println!(
                    "Password: {}{}{}",
                    password.chars().nth(0).unwrap(),
                    std::iter::repeat("*")
                        .take(password.len() - 2)
                        .collect::<String>(),
                    password.chars().last().unwrap()
                );
                let nm: NetworkManager = NetworkManager::new();
                if confirmation_prompt("Save password to RUSTPORT?", true) {
                    let status: u16 = match nm.add_password(&host_name, &password).await {
                        Ok(code) => code,
                        Err(_) => 500,
                    };
                    if status == 200 {
                        cprintln!("<green>Password saved</>");
                    } else {
                        cprintln!("<red>Something went wrong. Status code: {status}</>");
                    }
                }
            });
        });
        Ok(())
    }
}
