use std::process::exit;

use tokio::{runtime::Handle, task::block_in_place};

use crate::{cmd::Command, net::NetworkManager, utilities::ensure_auth};

pub struct RemoveCommand {
    #[allow(unused)]
    pub value: String,
}

impl Command for RemoveCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let _ = ensure_auth();
        let nm: NetworkManager = NetworkManager::new();

        block_in_place(move || {
            Handle::current().block_on(async move {
                let remove_status = match nm.remove(self.value.clone()).await {
                    Ok(val) => val,
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        exit(0);
                    }
                };
                if remove_status {
                    println!("Deleted password for '{}'", &self.value);
                }
            })
        });
        Ok(())
    }
}
