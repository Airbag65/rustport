use color_print::{ceprintln, cprintln};
use scanpw::scanpw;
use tokio::{runtime::Handle, task::block_in_place};

use crate::{
    UserInformation, cmd,
    net::{NetworkManager, login::LoginRes},
    utilities::{
        file::{get_local_information, save_local_auth, save_pem_string},
        read_input,
    },
};

pub struct LoginCommand;

impl cmd::Command for LoginCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let email = read_input("Email: ")?;
        let password = scanpw!("Password: ");
        println!();
        let local_info: UserInformation = get_local_information()?;
        block_in_place(move || {
            Handle::current().block_on(async move {
                let nm: NetworkManager = NetworkManager::new();
                let has_valid_token: bool = match nm.validate_token(&local_info.auth_token).await {
                    Ok(v) => v.to_owned(),
                    Err(e) => {
                        ceprintln!("<red>Something went wrong:</> {:?}", e);
                        return;
                    }
                };
                if has_valid_token {
                    cprintln!(
                        "<yellow>Already logged in with email '{}'</>",
                        local_info.email
                    );
                    return;
                }
                let res: LoginRes = nm.login(email.clone(), password).await.unwrap();
                match res.response_code {
                    200 => {
                        let _ = match save_pem_string(&res.pem_string) {
                            Ok(_) => {}
                            Err(e) => eprintln!("Error {:?}", e),
                        };
                        let _ = match save_local_auth(
                            &res.name,
                            &res.surname,
                            &res.email,
                            &res.auth_token,
                        ) {
                            Ok(_) => {
                                cprintln!(
                                    "<green>You are now logged in as '{} {}'</>",
                                    res.name,
                                    res.surname
                                );
                            }
                            Err(e) => eprintln!("Error {:?}", e),
                        };
                        // todo!("Save LOCAL AUTH");

                        // let _ = match save_local_auth(
                        //     &res.name,
                        //     &res.surname,
                        //     &res.email,
                        //     &res.auth_token,
                        // ) {};
                    }
                    404 => cprintln!("<yellow>Account with email '{}' does not exist</>", &email),
                    418 => cprintln!("<yellow>Already logged in with email '{}'</>", &email),
                    401 => cprintln!("<red>Incorrect password</>"),
                    _ => {}
                }
            })
        });
        Ok(())
    }
}
