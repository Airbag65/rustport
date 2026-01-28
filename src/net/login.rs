use serde::{Deserialize, Serialize};

use crate::net::NetworkManager;

#[derive(Serialize, Debug)]
#[allow(unused)]
struct LoginReq {
    email: String,
    password: String,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct LoginRes {
    response_code: u16,
    response_message: String,
    auth_token: String,
    name: String,
    surname: String,
    email: String,
    pem_string: String,
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
        // println!("{}", req_string);
        let res: reqwest::Response = self
            .client
            .post("https://localhost:443/auth/login")
            .header("Content-Type", "application/json")
            .body(req_string.clone())
            .send()
            .await?;
        let status_code: reqwest::StatusCode = res.status();

        // println!("{:?}", res);

        let mut res_obj: LoginRes;
        if status_code.as_u16() == 200 {
            res_obj = serde_json::from_str(res.text().await.unwrap().as_str())?;
            Ok(res_obj)
        } else {
            println!(
                "something went wrong. status was: {}. body: {:?}. req_body: {}",
                status_code.as_u16(),
                res.text().await.unwrap(),
                req_string
            );
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
