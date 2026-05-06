pub mod add;
pub mod edit;
pub mod get;
pub mod health;
pub mod list;
pub mod login;
pub mod remove;
pub mod reset;
pub mod signup;
pub mod singout;
pub mod valid;

#[derive(Clone)]
pub struct NetworkManager {
    client: reqwest::blocking::Client,
}

impl NetworkManager {
    pub fn new() -> NetworkManager {
        let client = match reqwest::blocking::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
        {
            Ok(client) => reqwest::blocking::Client::from(client),
            Err(_) => reqwest::blocking::Client::new(),
        };
        NetworkManager { client }
    }
}
