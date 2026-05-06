use std::process::exit;

use crate::{cmd::Command, net::NetworkManager, utilities::ensure_auth};

pub struct RemoveCommand {
    #[allow(unused)]
    pub value: String,
}

impl Command for RemoveCommand {
    fn execute(&self) -> Result<(), anyhow::Error> {
        let _ = ensure_auth();
        let nm: NetworkManager = NetworkManager::new();

        let remove_status = match nm.remove(self.value.clone()) {
            Ok(val) => val,
            Err(e) => {
                eprintln!("Error: {}", e);
                exit(0);
            }
        };
        if remove_status {
            println!("Deleted password for '{}'", &self.value);
        }
        Ok(())
    }
}
