mod auth;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let local_user = match auth::get_local_information() {
        Ok(person) => person,
        Err(e) => return Err(e),
    };
    println!("{}", local_user);
    Ok(())
}
