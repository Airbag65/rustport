use serde::{Deserialize, Serialize};

use crate::{net::NetworkManager, utilities::get_ip};

#[derive(Serialize, Debug)]
#[allow(unused)]
struct LoginReq {
    email: String,
    password: String,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct LoginRes {
    pub response_code: u16,
    pub response_message: String,
    pub auth_token: String,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub pem_string: String,
}

impl NetworkManager {
    #[allow(unused)]
    pub async fn login(
        &self,
        email: String,
        password: String,
    ) -> Result<LoginRes, Box<dyn std::error::Error>> {
        let req_body: LoginReq = LoginReq {
            email: email,
            password: password,
        };
        let req_string: String = serde_json::to_string(&req_body)?;
        let res: reqwest::Response = self
            .client
            .post("https://".to_owned() + get_ip().as_str() + ":443/auth/login")
            .header("Content-Type", "application/json")
            .body(req_string.clone())
            .send()
            .await?;
        let status_code: reqwest::StatusCode = res.status();

        let mut res_obj: LoginRes;
        if status_code.as_u16() == 200 {
            res_obj = serde_json::from_str(res.text().await.unwrap().as_str())?;
            Ok(res_obj)
        } else {
            Ok(LoginRes {
                response_code: status_code.as_u16(),
                response_message: String::new(),
                auth_token: String::new(),
                name: String::new(),
                surname: String::new(),
                email: String::new(),
                pem_string: String::new(),
            })
        }
    }
}
