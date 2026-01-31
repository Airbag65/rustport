use color_print::{ceprintln, cprintln};
use tokio::{runtime::Handle, task::block_in_place};

use crate::{
    cmd::Command,
    net::{NetworkManager, singout::SignOutRes},
    utilities::file::remove_local_auth,
};

pub struct LogoutCommand;

impl Command for LogoutCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        (move || {
            Handle::current().block_on(async move {
                let nm: NetworkManager = NetworkManager::new();
                let response: SignOutRes = match nm.sign_out().await {
                    Ok(res) => res,
                    Err(e) => {
                        ceprintln!("<red>Something went wrong! Error: {:?}</>", e);
                        return;
                    }
                };
                match response.response_code {
                    200 => {
                        remove_local_auth().unwrap();
                        cprintln!("<green>You are now signed out</>");
                    }
                    304 => cprintln!("<yellow>You were already signed out</>"),
                    _ => {}
                }
            })
        });
        Ok(())
    }
}
