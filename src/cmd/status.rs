use color_print::{ceprintln, cprintln};
use tokio::{runtime::Handle, task::block_in_place};

use crate::{cmd, net::NetworkManager, utilities::file::get_local_information};

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
                    Err(e) => {
                        ceprintln!("<red>Something went wrong:</> {:?}", e);
                        return;
                    }
                };
                if res {
                    cprintln!("<green>You are signed in to RUSTPORT! RUSTPORT is ready to use</>");
                    println!("Your credentials:");
                    println!("-----------------");
                    println!("Name: {} {}", local_info.name, local_info.surname);
                    println!("Email: {}", local_info.email);
                } else {
                    cprintln!("<red>You are not signed in to RUSTPORT!</>");
                    cprintln!("<red>Run 'rustport login' to sign in</>");
                }
            })
        });
        Ok(())
    }
}
