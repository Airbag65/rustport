use reqwest::StatusCode;
use serde::Serialize;
use serde_json;

use crate::{net::NetworkManager, utilities::get_ip};

#[derive(Serialize, Debug)]
struct ValidateTokenReq {
    auth_token: String,
    email: String,
}

impl NetworkManager {
    pub fn validate_token(&self, auth_token: &str, email: &str) -> Result<bool, anyhow::Error> {
        let req_body: ValidateTokenReq = ValidateTokenReq {
            auth_token: String::from(auth_token),
            email: String::from(email),
        };
        let req_string: String = serde_json::to_string(&req_body)?;
        let res: reqwest::blocking::Response = self
            .client
            .post("https://".to_owned() + get_ip().as_str() + ":443/auth/valid")
            .header("Content-Type", "application/json")
            .body(req_string.clone())
            .send()?;
        let status: StatusCode = res.status();
        if status.as_u16() == 200 {
            return Ok(true);
        }
        Ok(false)
    }
}
