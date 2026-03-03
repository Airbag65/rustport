use std::process::exit;

use color_print::cprintln;
use scanpw::scanpw;
use tokio::{runtime::Handle, task::block_in_place};

use crate::{
    cmd::Command,
    net::NetworkManager,
    utilities::{confirmation_prompt, ensure_auth, print_boxed},
};

pub struct EditCommand {
    pub value: String,
}

impl Command for EditCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let nm: NetworkManager = NetworkManager::new();
        block_in_place(move || {
            Handle::current().block_on(async move {
                let token: String = ensure_auth();
                let possible_hosts = nm.list(&token).await.unwrap();
                print!("\x1B[2J\x1B[1;1H");
                if !possible_hosts
                    .hosts
                    .iter()
                    .any(|host| self.value.contains(host))
                {
                    cprintln!("<red>No password for '{}' found</>", self.value);
                    exit(0);
                }
                cprintln!("<green>Password for '{}' has been found</>", &self.value);
                let mut new_password: String;
                let mut new_confirm_password: String;
                loop {
                    new_password = scanpw!("Password: ");
                    new_confirm_password = scanpw!("Confirm password: ");
                    if new_password == new_confirm_password {
                        break;
                    }
                    cprintln!("<red>Passwords don't match</>");
                }

                if confirmation_prompt("Would you like to display the new password?", false) {
                    print_boxed(&new_password);
                }
                println!("\nA password is about to be updated!");
                println!("------------------------------------");
                println!("Host: {}", &self.value);
                println!(
                    "Password: {}{}{}",
                    new_password.chars().nth(0).unwrap(),
                    std::iter::repeat("*")
                        .take(new_password.len() - 2)
                        .collect::<String>(),
                    new_password.chars().last().unwrap()
                );
                if confirmation_prompt("Update password?", true) {
                    let res: bool = nm
                        .edit_password(String::from(&self.value), new_password)
                        .await
                        .unwrap();
                    if !res {
                        cprintln!("<red>Something went wrong!</>");
                        exit(0);
                    }
                }
            })
        });
        Ok(())
    }
}
