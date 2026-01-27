use std::env;

mod auth;
mod parse;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    parse::say_hello();
    let local_user = match auth::get_local_information() {
        Ok(person) => person,
        Err(e) => return Err(e),
    };
    println!("{}", local_user);
    for arg in env::args() {
        println!("{arg}");
    }
    Ok(())
}
