use std::{
    io::{self, Write},
    process::exit,
    thread, time,
};

use color_print::cprintln;
use tokio::{runtime::Handle, task::block_in_place};

use crate::{
    cmd::Command,
    net::NetworkManager,
    utilities::{confirmation_prompt, ensure_auth, print_boxed},
};

#[allow(unused)]
pub struct GetCommand {
    pub value: String,
}

impl Command for GetCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let nm: NetworkManager = NetworkManager::new();
        block_in_place(move || {
            Handle::current().block_on(async move {
                let token = ensure_auth();
                let possible_hosts = nm.list(&token).await.unwrap();
                if !possible_hosts.hosts.iter().any(|host| self.value.contains(host)) {
                    cprintln!("<red>No password for '{}' found", self.value);
                    exit(0);
                }
                let password: String = nm.get(self.value.clone()).await.unwrap_or("".to_owned());
                cprintln!("<green>Password for '{}' has been found</>", &self.value);
                cprintln!("<red>WARNING! Be careful when revealing the password. The password will be printed to the terminal window</>");
                if confirmation_prompt("Would you like to display the password?", false) {
                    print_boxed(&password);
                }
                println!("Terminal window clearing in ");
                let _ = io::stdout().flush();
                let sec = time::Duration::from_secs(1);
                for n in 0..10 {
                    print!("\r{} ", 10 - n);
                    let _ = io::stdout().flush();
                    thread::sleep(sec);
                }
                // Clear the terminal window ANSI escape code
                print!("\x1B[2J\x1B[1;1H");
            })
        });
        Ok(())
    }
}
