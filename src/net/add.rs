use base64::{Engine, engine::general_purpose::STANDARD};
use rand::thread_rng;
use rsa::{Oaep, RsaPublicKey, pkcs1::DecodeRsaPublicKey};
use serde::Serialize;
use sha2::Sha256;

use crate::{
    net::NetworkManager,
    utilities::{ensure_auth, file::read_file, get_ip},
};

#[derive(Serialize, Debug)]
#[allow(unused)]
struct AddPasswordReq {
    host_name: String,
    password: String,
}

impl NetworkManager {
    pub async fn add_password(
        &self,
        host_name: &str,
        password: &str,
    ) -> Result<u16, Box<dyn std::error::Error>> {
        let token: String = ensure_auth();

        // TODO: RSA encryption -> BASE64 String
        let mut rng = thread_rng();
        let pem_string = read_file("publicKey.pem").unwrap();
        let key = RsaPublicKey::from_pkcs1_pem(&pem_string)?;
        let padding = Oaep::new::<Sha256>();
        let enc_bytes = key.encrypt(&mut rng, padding, password.as_bytes()).unwrap();
        let enc_password = STANDARD.encode(&enc_bytes);

        let req: AddPasswordReq = AddPasswordReq {
            host_name: String::from(host_name),
            password: String::from(enc_password),
        };
        let req_string: String = serde_json::to_string(&req)?;
        let res: reqwest::Response = self
            .client
            .post("https://".to_owned() + get_ip().as_str() + ":443/pwd/new")
            .header("Authorization", "Bearer ".to_owned() + token.as_str())
            .header("Content-Type", "application/json")
            .body(req_string)
            .send()
            .await?;
        Ok(res.status().as_u16())
    }
}
