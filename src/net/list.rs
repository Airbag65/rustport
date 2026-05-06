use serde::Deserialize;

use crate::{net::NetworkManager, utilities::get_ip};

#[derive(Deserialize, Debug)]
pub struct ListRes {
    pub hosts: Vec<String>,
}

impl NetworkManager {
    pub fn list(&self, token: &str) -> Result<ListRes, anyhow::Error> {
        let res: reqwest::blocking::Response = self
            .client
            .get("https://".to_owned() + get_ip().as_str() + ":443/pwd/getHosts")
            .header("Authorization", "Bearer ".to_owned() + token)
            .send()
            .unwrap();
        let hosts: ListRes = serde_json::from_str(res.text()?.as_str())?;
        Ok(hosts)
    }
}
