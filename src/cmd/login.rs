use std::{
    fmt::Display,
    io::{self, Write},
};

use color_print::{ceprintln, cprintln};
use scanpw::scanpw;
use tokio::{runtime::Handle, task::block_in_place};

use crate::{
    UserInformation, cmd, get_local_information,
    net::{NetworkManager, login::LoginRes},
};

pub struct LoginCommand;

impl cmd::Command for LoginCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Login command");
        let email = read_input("Email: ")?;
        let password = scanpw!("Password: ");
        println!();
        let local_info: UserInformation = get_local_information()?;

        block_in_place(move || {
            Handle::current().block_on(async move {
                let nm: NetworkManager = NetworkManager::new();
                let res: bool = match nm.validate_token(&local_info.auth_token).await {
                    Ok(v) => v.to_owned(),
                    Err(e) => {
                        ceprintln!("<red>Something went wrong:</> {:?}", e);
                        return;
                    }
                };
                if res {
                    cprintln!(
                        "<yellow>Already logged in with email '{}'</>",
                        local_info.email
                    );
                    return;
                }
                let res: LoginRes = nm.login(email, password).await.unwrap();
                println!("{:?}", res);
                todo!("Save PEM string, save local auth")
            })
        });
        Ok(())
    }
}

fn read_input(prompt: impl Display) -> Result<String, Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    print!("{prompt}");
    let _ = std::io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer)?;
    buffer = buffer.trim_ascii_end().to_owned();
    Ok(buffer)
}
