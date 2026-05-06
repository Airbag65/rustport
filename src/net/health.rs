use std::process::exit;

use serde::Deserialize;

use crate::{net::NetworkManager, utilities::get_ip};

#[derive(Deserialize)]
#[allow(unused)]
pub struct HealthRes {
    pub health: String,
    pub rustport_version: String,
}

#[allow(unused)]
impl NetworkManager {
    pub fn health(&self) -> Result<HealthRes, anyhow::Error> {
        let res: reqwest::blocking::Response = self
            .client
            .get("https://".to_owned() + get_ip().as_str() + "/status")
            .send()?;

        if res.status() != 200 {
            exit(0);
        }
        let response: HealthRes = serde_json::from_str(res.text()?.as_str())?;
        Ok(response)
    }
}
