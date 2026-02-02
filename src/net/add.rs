use std::process::exit;

use serde::Serialize;

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
        let pem_string = read_file("publicKey.pem").unwrap();
        println!("{pem_string}");
        let public_key = match openssl::rsa::Rsa::public_key_from_pem_pkcs1(pem_string.as_bytes()) {
            Ok(key) => key,
            Err(e) => {
                eprintln!("Error: {e}");
                exit(0);
            }
        };
        let mut enc_data = [0; 1024];
        public_key.public_encrypt(
            password.as_bytes(),
            &mut enc_data,
            openssl::rsa::Padding::PKCS1_OAEP,
        )?;
        println!("{:?}", enc_data);
        // TODO: turn bytes to string
        let enc_password = str::from(&enc_data);
        println!("{:?}", enc_password);

        // let req: AddPasswordReq = AddPasswordReq {
        //     host_name: String::from(host_name),
        //     password: String::from(password),
        // };
        // let req_string: String = serde_json::to_string(&req)?;
        // println!("{req_string}");
        // let res: reqwest::Response = self
        //     .client
        //     .post("https://".to_owned() + get_ip().as_str() + ":443/pwd/new")
        //     .header("Authorization", "Bearer ".to_owned() + token.as_str())
        //     .header("Content-Type", "application/json")
        //     .body(req_string)
        //     .send()
        //     .await?;
        // Ok(res.status().as_u16())
        Ok(200)
    }
}
