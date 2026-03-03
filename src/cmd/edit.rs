use std::process::exit;

use color_print::cprintln;
use tokio::{runtime::Handle, task::block_in_place};

use crate::{cmd::Command, net::NetworkManager, utilities::ensure_auth};

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
                if !possible_hosts
                    .hosts
                    .iter()
                    .any(|host| self.value.contains(host))
                {
                    cprintln!("<red>No password for '{}' found</>", self.value);
                    exit(0);
                }
            })
        });

        Ok(())
    }
}
