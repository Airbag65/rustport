use crate::{net::NetworkManager, utilities::get_ip};
use anyhow::anyhow;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
struct RegisterReq {
    email: String,
    name: String,
    surname: String,
    password: String,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct RegisterRes {
    pub response_code: u16,
    pub response_message: String,
    pub auth_token: String,
    pub name: String,
    pub surname: String,
    pub pem_string: String,
}

impl NetworkManager {
    #[allow(unused)]
    pub fn sign_up(
        &self,
        name: &str,
        surname: &str,
        email: &str,
        password: &str,
    ) -> Result<RegisterRes, anyhow::Error> {
        if name == "" || surname == "" || email == "" || password == "" {
            return Err(anyhow!("Insufficient information provided"));
        }
        let req: RegisterReq = RegisterReq {
            email: String::from(email),
            name: String::from(name),
            surname: String::from(surname),
            password: String::from(password),
        };
        let req_string: String = serde_json::to_string(&req)?;

        let res: reqwest::blocking::Response = self
            .client
            .post("https://".to_owned() + get_ip().as_str() + ":443/auth/new")
            .header("Content-Type", "application/json")
            .body(req_string.clone())
            .send()?;

        if res.status().as_u16() != 200 {
            return Err(anyhow!("Status code was not 200"));
        }
        let reg_res: RegisterRes = serde_json::from_str(&res.text()?)?;

        Ok(reg_res)
    }
}
