use serde::{Deserialize, Serialize};

use crate::{
    net::NetworkManager,
    utilities::{ensure_auth, get_ip},
};

#[derive(Serialize, Debug)]
#[allow(unused)]
struct GetPasswordReq {
    host_name: String,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct GetPasswordRes {
    pub password: String,
}

impl NetworkManager {
    #[allow(unused)]
    pub async fn get(&self, host_name: String) -> Result<String, Box<dyn std::error::Error>> {
        let token: String = ensure_auth();
        let req: GetPasswordReq = GetPasswordReq { host_name };
        let req_string: String = serde_json::to_string(&req)?;
        let res: reqwest::Response = self
            .client
            .put("https://".to_owned() + get_ip().as_str() + ":443/pwd/get")
            .header("Authorization", "Bearer ".to_owned() + &token)
            .body(req_string)
            .send()
            .await?;
        let res_obj: GetPasswordRes = serde_json::from_str(res.text().await?.as_str())?;

        Ok(res_obj.password.to_owned())
    }
}
