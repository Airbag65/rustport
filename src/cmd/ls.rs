use std::process::exit;

use color_print::{ceprintln, cprintln};

use crate::{
    cmd::Command,
    net::{NetworkManager, list::ListRes},
    utilities::{convert_host, ensure_auth},
};

pub struct LsCommand;

impl Command for LsCommand {
    fn execute(&self) -> Result<(), anyhow::Error> {
        #[allow(unused)]
        let token = ensure_auth();
        let nm: NetworkManager = NetworkManager::new();
        let res: ListRes = match nm.list(&token) {
            Ok(l) => l,
            Err(e) => {
                ceprintln!("<red>Something went wrong! Error: {e}</>");
                exit(0);
            }
        };
        for host in res.hosts {
            let converted: String = convert_host(host.clone());
            cprintln!("{} <i><rgb(200, 200, 200)>use: {}</></>", converted, host);
        }

        Ok(())
    }
}
