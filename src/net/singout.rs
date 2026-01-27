use reqwest::StatusCode;
use serde::Serialize;
use serde_json;
use std;

#[derive(Serialize, Debug)]
struct ValidateTokenReq {
    auth_token: String,
}

pub async fn validate_token(auth_token: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let req_body: ValidateTokenReq = ValidateTokenReq {
        auth_token: String::from(auth_token),
    };
    let req_string: String = serde_json::to_string(&req_body)?;
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;
    let res = client
        .post("https://127.0.0.1:443/auth/valid")
        .header("Content-Type", "application/json")
        .body(req_string.clone())
        .send()
        .await?;
    let status: StatusCode = res.status();
    if status.as_u16() == 200 {
        return Ok(true);
    }
    Ok(false)
}
