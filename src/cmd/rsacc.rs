use std::process::exit;

use color_print::cprintln;
use tokio::{runtime::Handle, task::block_in_place};

use crate::{
    cmd::Command,
    net::NetworkManager,
    utilities::{ensure_auth, read_input},
};

#[allow(unused)]
pub struct ResetAccountCommand;

impl Command for ResetAccountCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let token: String = ensure_auth();
        if token != "" {
            cprintln!("<yellow>You cannot be signed in while resetting an account</>");
            exit(0);
        }
        let email: String = read_input("Email for account to reset: ")?;
        block_in_place(move || {
            Handle::current().block_on(async move {
                let nm: NetworkManager = NetworkManager::new();
                let success: bool = nm.reset_account(email.clone()).await;

                if success {
                    cprintln!("<green>A reset request has been sent! Check the inbox mail inbox for '{}' to proceed!</>", email);
                } else {
                    cprintln!("<red>Something went wrong!</>");
                    println!("What might have gone wrong:\n\t* No account with email '{}' exists\n\t* Could not send email\n\t* Server is down", email);
                }
            })
        });
        Ok(())
    }
}
