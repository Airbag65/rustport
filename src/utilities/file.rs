use std::{
    env,
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
    process::exit,
};

use crate::{Alias, Config, UserInformation};

pub fn get_local_information() -> Result<UserInformation, Box<dyn std::error::Error>> {
    let home_dir = match env::home_dir() {
        Some(path) => path,
        None => PathBuf::new(),
    };

    let home_str: &str = home_dir.to_str().unwrap();
    let full_path = String::from(home_str) + "/.passport/authentication.json";
    let file: File = match File::open(&full_path) {
        Ok(f) => f,
        Err(_) => {
            fs::create_dir(PathBuf::from(String::from(home_str) + "/.passport"))?;
            let f = fs::File::create(full_path)?;
            f
        }
    };
    let reader = BufReader::new(file);
    let user: UserInformation = serde_json::from_reader(reader)?;
    Ok(user)
}

pub fn get_configuration() -> Result<Config, Box<dyn std::error::Error>> {
    let home_dir: PathBuf = match env::home_dir() {
        Some(path) => path,
        None => exit(0),
    };
    let home_str: &str = home_dir.to_str().unwrap();
    let full_path: String = String::from(home_str) + "/.passport/config.toml";
    let config_content: String = read_file(&full_path)?;
    let mut config: Config = match toml::from_str(&config_content) {
        Ok(v) => v,
        Err(_) => Config {
            global: crate::Global {
                source_path: ".".to_owned(),
                ip_addr: "127.0.0.1".to_owned(),
            },
            alias: None,
        },
    };
    match config.alias {
        Some(_) => {}
        None => {
            config.alias = Some(Alias {
                list: vec![],
                add: vec![],
                edit: vec![],
                generate: vec![],
                get: vec![],
                help: vec![],
                version: vec![],
                init: vec![],
                login: vec![],
                signout: vec![],
                register: vec![],
                remove: vec![],
                reset_account: vec![],
                status: vec![],
                alias: vec![],
                view: vec![],
            });
            let _ = update_config(&config);
        }
    };
    Ok(config)
}

pub fn update_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let config_string = toml::to_string(&config)?;
    write_file("config.toml", &config_string)?;
    Ok(())
}

pub fn save_pem_string(pem_string: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut path: PathBuf = env::home_dir().ok_or("Could not find home directory")?;
    path.push(".passport");
    fs::create_dir_all(&path)?;
    path.push("publicKey.pem");
    fs::write(path, pem_string)?;
    Ok(())
}

pub fn save_local_auth(
    name: &str,
    surname: &str,
    email: &str,
    auth_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let user: UserInformation = UserInformation {
        name: String::from(name),
        surname: String::from(surname),
        email: String::from(email),
        auth_token: String::from(auth_token),
    };

    let mut path: PathBuf = env::home_dir().ok_or("Could not find home directory")?;
    path.push(".passport");
    fs::create_dir_all(&path)?;
    path.push("authentication.json");
    fs::write(path, serde_json::to_string(&user)?)?;
    Ok(())
}

#[allow(unused)]
pub fn remove_local_auth() -> Result<(), Box<dyn std::error::Error>> {
    let mut path: PathBuf = env::home_dir().ok_or("Could not find home directory")?;
    path.push(".passport");
    fs::create_dir_all(&path)?;
    path.push("authentication.json");
    let empty_user: UserInformation = UserInformation {
        name: String::new(),
        surname: String::new(),
        email: String::new(),
        auth_token: String::new(),
    };
    fs::write(path, serde_json::to_string(&empty_user)?)?;
    Ok(())
}

pub fn write_file(filename: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut path: PathBuf = env::home_dir().ok_or("Could not find home directory")?;
    path.push(".passport");
    fs::create_dir_all(&path)?;
    path.push(filename);
    fs::write(path, content)?;
    Ok(())
}

pub fn read_file(filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut path: PathBuf = env::home_dir().ok_or("Could not find home directory")?;
    path.push(".passport");
    fs::create_dir_all(&path)?;
    path.push(filename);
    let content: String = fs::read_to_string(path)?;
    Ok(content.clone())
}
