use serde::Deserialize;

use crate::{net::NetworkManager, utilities::get_ip};

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct ListRes {
    pub hosts: Vec<String>,
}

impl NetworkManager {
    pub async fn list(&self, token: &str) -> Result<ListRes, Box<dyn std::error::Error>> {
        let res: reqwest::Response = self
            .client
            .get("https://".to_owned() + get_ip().as_str() + ":443/pwd/getHosts")
            .header("Authorization", "Bearer ".to_owned() + token)
            .send()
            .await
            .unwrap();
        let hosts: ListRes = serde_json::from_str(res.text().await?.as_str())?;
        Ok(hosts)
    }
}
