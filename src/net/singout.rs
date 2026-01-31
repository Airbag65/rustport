use serde::{Deserialize, Serialize};

use crate::{
    net::NetworkManager,
    utilities::{file::get_local_information, get_ip},
};

#[derive(Serialize, Debug)]
#[allow(unused)]
struct SignOutReq {
    email: String,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct SignOutRes {
    pub response_code: u16,
    pub response_message: String,
}

impl NetworkManager {
    #[allow(unused)]
    pub async fn sign_out(&self) -> Result<SignOutRes, Box<dyn std::error::Error>> {
        let email: String = get_local_information()?.email;
        let req: SignOutReq = SignOutReq { email };
        let req_string: String = serde_json::to_string(&req)?;
        let res: reqwest::Response = self
            .client
            .put("https://".to_owned() + get_ip().as_str() + ":443/auth/signOut")
            .header("Content-Type", "application/json")
            .body(req_string.clone())
            .send()
            .await?;
        let status_code: reqwest::StatusCode = res.status();
        let sign_out_res: SignOutRes = serde_json::from_str(res.text().await.unwrap().as_str())?;
        Ok(sign_out_res)
    }
}
