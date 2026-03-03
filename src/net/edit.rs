use base64::{Engine, engine::general_purpose::STANDARD};
use rand::thread_rng;
use rsa::{Oaep, RsaPublicKey, pkcs1::DecodeRsaPublicKey};
use serde::Serialize;
use sha2::Sha256;

use crate::{
    net::NetworkManager,
    utilities::{ensure_auth, file::read_file, get_ip},
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

        let mut rng = thread_rng();
        let pem_string = read_file("publicKey.pem").unwrap();
        let key = RsaPublicKey::from_pkcs1_pem(&pem_string)?;
        let padding = Oaep::new::<Sha256>();
        let enc_bytes = key
            .encrypt(&mut rng, padding, new_password.as_bytes())
            .unwrap();
        let enc_password = STANDARD.encode(&enc_bytes);
        let req: EditPasswordReq = EditPasswordReq {
            host_name,
            new_password: enc_password,
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
