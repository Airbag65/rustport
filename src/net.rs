pub mod valid;

pub struct NetworkManager {
    client: reqwest::Client,
}

impl NetworkManager {
    #[allow(dead_code)]
    pub fn new() -> NetworkManager {
        let client = match reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
        {
            Ok(client) => reqwest::Client::from(client),
            Err(_) => reqwest::Client::new(),
        };
        let nm: NetworkManager = NetworkManager { client };
        nm
    }
}
