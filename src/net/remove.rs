use std::process::exit;

use color_print::cprintln;
use serde::Serialize;

use crate::{net::NetworkManager, utilities::ensure_auth};

#[derive(Serialize, Debug)]
#[allow(unused)]
struct RemoveReq {
    host_name: String,
}

impl NetworkManager {
    #[allow(unused)]
    pub async fn remove(&self, host_name: String) -> Result<bool, Box<dyn std::error::Error>> {
        let token = ensure_auth();
        let list = self.list(&token).await?;
        if !list.hosts.iter().any(|host| host_name.contains(host)) {
            cprintln!(
                "<red>Password for '{}' does not exist\nRun 'rustport list' or 'rustport ls' to see available passwords",
                &host_name
            );
            exit(0);
        }

        Ok(true)
    }
}
