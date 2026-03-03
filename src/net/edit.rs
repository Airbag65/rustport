use serde::Serialize;

use crate::{
    net::NetworkManager,
    utilities::{ensure_auth, get_ip},
};

#[allow(unused)]
#[derive(Serialize)]
pub struct EditPasswordReq {
    pub host_name: String,
    pub new_password: String,
}

impl NetworkManager {
    #[allow(unused)]
    pub async fn edit_password(
        &self,
        host_name: String,
        new_password: String,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let token: String = ensure_auth();
        let req: EditPasswordReq = EditPasswordReq {
            host_name,
            new_password,
        };
        let req_string: String = serde_json::to_string(&req)?;
        let res: reqwest::Response = self
            .client
            .put("https://".to_owned() + get_ip().as_str() + ":443/pwd/edit")
            .header("Authorization", "Bearer ".to_owned() + token.as_str())
            .body(req_string)
            .send()
            .await?;
        if res.status().as_u16() == 200 {
            return Ok(true);
        }
        Ok(false)
    }
}
