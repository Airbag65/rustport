use std::{env, fs, path::PathBuf, process::exit, thread};

use tokio::{runtime::Handle, task::block_in_place};

use crate::{Config, cmd::Command, net::NetworkManager, utilities::file::get_configuration};

pub struct UpdateCommand;

impl Command for UpdateCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut rp_version: String = String::new();
        let nm: NetworkManager = NetworkManager::new();
        block_in_place(|| {
            Handle::current().block_on(async {
                let health = nm.health().await.unwrap();
                rp_version = health.rustport_version;
            })
        });
        if rp_version == env!("CARGO_PKG_VERSION") {
            println!("Already at the latest rustport version! Nothing to do.");
            exit(0);
        }
        let config: Config = get_configuration()?;
        env::set_current_dir(&config.global.source_path)?;
        block_in_place(|| {
            Handle::current().block_on(async {
                let git_pull = async_process::Command::new("git")
                    .arg("pull")
                    .output()
                    .await;
                if !git_pull.is_ok() {
                    println!("Something went wrong (git pull)");
                    exit(0);
                }
                let cargo_build = async_process::Command::new("cargo")
                    .arg("build")
                    .arg("--release")
                    .output()
                    .await;
                if !cargo_build.unwrap().status.success() {
                    println!("Something went wrong (cargo build --release)");
                    exit(0);
                }
            })
        });
        let home_dir = match env::home_dir() {
            Some(path) => path,
            None => PathBuf::new(),
        };
        let home_str = home_dir.to_str().unwrap().to_string();

        match fs::copy(
            String::from(&config.global.source_path) + "/target/release/rp",
            String::from(&home_str) + "/.cargo/bin/rp",
        ) {
            Ok(_) => {}
            Err(_) => {}
        };
        match fs::copy(
            String::from(&config.global.source_path) + "/target/release/rp",
            String::from(&home_str) + "/.cargo/bin/rustport",
        ) {
            Ok(_) => {}
            Err(_) => {}
        };
        println!("Rustport has been updated to version {}", rp_version);
        exit(0);
    }
}
