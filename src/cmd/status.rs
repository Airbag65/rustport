use std::process::exit;

use color_print::{ceprintln, cprintln};
use tokio::{runtime::Handle, task::block_in_place};

use crate::{
    cmd,
    net::NetworkManager,
    utilities::{file::get_local_information, get_ip},
};

pub struct StatusCommand;

impl cmd::Command for StatusCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        #[allow(unused)]
        let nm: NetworkManager = NetworkManager::new();
        let local_info = match get_local_information() {
            Ok(info) => info,
            Err(e) => return Err(e),
        };
        block_in_place(move || {
            Handle::current().block_on(async move {
                let res: bool = match nm.validate_token(&local_info.auth_token).await {
                    Ok(v) => v,
                    Err(_) => {
                        ceprintln!("<red>Could not connect to server</>");
                        println!("Using IP: {}", get_ip());
                        exit(0);
                    }
                };
                if res {
                    cprintln!("<green>You are signed in to RUSTPORT! RUSTPORT is ready to use</>");
                    println!("Your credentials:");
                    println!("-----------------");
                    println!("Name: {} {}", local_info.name, local_info.surname);
                    println!("Email: {}", local_info.email);
                    println!("Using IP: {}", get_ip());
                } else {
                    cprintln!("<red>You are not signed in to RUSTPORT!</>");
                    cprintln!("<red>Run 'rustport login' to sign in</>");
                    println!("Using IP: {}", get_ip());
                }
            })
        });
        Ok(())
    }
}
