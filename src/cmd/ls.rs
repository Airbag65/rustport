use std::process::exit;

use color_print::ceprintln;
use tokio::{runtime::Handle, task::block_in_place};

use crate::{
    cmd::Command,
    net::{NetworkManager, list::ListRes},
    utilities::ensure_auth,
};

pub struct LsCommand;

impl Command for LsCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        #[allow(unused)]
        let token = ensure_auth();
        let nm: NetworkManager = NetworkManager::new();
        block_in_place(move || {
            Handle::current().block_on(async move {
                let res: ListRes = match nm.list(&token).await {
                    Ok(l) => l,
                    Err(e) => {
                        ceprintln!("<red>Something went wrong! Error: {e}</>");
                        exit(0);
                    }
                };
                for host in res.hosts {
                    println!("{host}");
                }
            })
        });

        Ok(())
    }
}
