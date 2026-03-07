use serde::Serialize;

use crate::{net::NetworkManager, utilities::get_ip};

#[allow(unused)]
#[derive(Serialize)]
pub struct ResetAccountRequest {
    pub email: String,
}

impl NetworkManager {
    #[allow(unused)]
    pub async fn reset_account(&self, email: String) -> bool {
        let req: ResetAccountRequest = ResetAccountRequest { email };
        let req_string: String = match serde_json::to_string(&req) {
            Ok(v) => v,
            Err(_) => return false,
        };
        let res: reqwest::Response = match self
            .client
            .put("https://".to_owned() + get_ip().as_str() + ":443/auth/reset")
            .header("Content-Type", "application/json")
            .body(req_string)
            .send()
            .await
        {
            Ok(v) => v,
            Err(_) => return false,
        };
        if res.status() == 200 {
            return true;
        }
        false
    }
}
