use std::process::exit;

use serde::Deserialize;

use crate::{net::NetworkManager, utilities::get_ip};

#[derive(Deserialize)]
#[allow(unused)]
pub struct HealthRes {
    pub health: String,
    pub rustport_version: String,
}

impl NetworkManager {
    pub async fn health(&self) -> Result<HealthRes, Box<dyn std::error::Error>> {
        let res: reqwest::Response = self
            .client
            .get("https://".to_owned() + get_ip().as_str() + "/status")
            .send()
            .await?;

        if res.status() != 200 {
            exit(0);
        }
        let response: HealthRes = serde_json::from_str(res.text().await?.as_str())?;
        Ok(response)
    }
}
