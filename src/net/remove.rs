use std::process::exit;

use color_print::cprintln;
use serde::Serialize;

use crate::{
    net::NetworkManager,
    utilities::{confirmation_prompt, ensure_auth, get_ip},
};

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

        let req: RemoveReq = RemoveReq {
            host_name: host_name.clone(),
        };
        let req_string: String = serde_json::to_string(&req)?;

        if !confirmation_prompt(
            format!(
                "Are you sure that you want to delete the possword for '{}'",
                &host_name
            ),
            false,
        ) {
            exit(0);
        }

        let res: reqwest::Response = self
            .client
            .delete("https://".to_owned() + get_ip().as_str() + ":443/pwd/remove")
            .header("Authorization", String::from(format!("Bearer {}", token)))
            .body(req_string)
            .send()
            .await?;
        let status = res.status();
        if status.as_u16() == 200 {
            return Ok(true);
        }

        Ok(false)
    }
}
