use std::process::exit;

use color_print::{ceprintln, cprintln};
use scanpw::scanpw;
use tokio::{runtime::Handle, task::block_in_place};

use crate::{
    UserInformation,
    cmd::Command,
    net::{NetworkManager, signup::RegisterRes},
    utilities::{
        file::{get_local_information, save_local_auth},
        read_input,
    },
};

pub struct RegisterCommand;

impl Command for RegisterCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Sign up new user");
        println!("----------------");
        let email: String = read_input("Email: ")?;
        let name: String = read_input("First name: ")?;
        let surname: String = read_input("Surname: ")?;
        let mut password: String;
        let mut confirm_password: String;
        loop {
            password = scanpw!("Password: ");
            println!();
            confirm_password = scanpw!("Confirm password: ");
            println!();
            if password == confirm_password {
                break;
            }
            cprintln!("<red>Passwords don't match</>");
        }
        block_in_place(move || {
            Handle::current().block_on(async move {
                let nm: NetworkManager = NetworkManager::new();
                let local_auth: UserInformation = match get_local_information() {
                    Ok(some) => some,
                    Err(e) => {
                        ceprintln!("<red>Something went wrong! Error: {}</>", e);
                        exit(0)
                    }
                };
                let has_valid_token = match nm.validate_token(&local_auth.auth_token).await {
                    Ok(v) => v.to_owned(),
                    Err(e) => {
                        ceprintln!("<red>Something went wrong! Error: {}</>", e);
                        exit(0)
                    }
                };
                if has_valid_token {
                    let _ = nm.sign_out().await.unwrap();
                    cprintln!(
                        "<green>Signing out '{} {}'",
                        local_auth.name,
                        local_auth.surname
                    );
                }
                let sign_up_res: RegisterRes = nm
                    .sign_up(
                        name.clone().as_str(),
                        surname.clone().as_str(),
                        email.clone().as_str(),
                        password.clone().as_str(),
                    )
                    .await
                    .unwrap();
                save_local_auth(
                    &sign_up_res.name,
                    &sign_up_res.surname,
                    &email,
                    &sign_up_res.auth_token,
                )
                .unwrap();
                match sign_up_res.response_code {
                    200 => {
                        cprintln!(
                            "<green>Created new user '{} {}' with email '{}'\n",
                            sign_up_res.name,
                            sign_up_res.surname,
                            email
                        );
                        cprintln!(
                            "<green>Signed in as '{} {}'",
                            sign_up_res.name,
                            sign_up_res.surname
                        );
                    }
                    418 => {
                        cprintln!("<yellow>User with email '{}' already exists", email);
                    }
                    _ => {}
                };
            })
        });

        Ok(())
    }
}
