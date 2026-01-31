use color_print::cprintln;

use crate::{
    cmd,
    utilities::{file::write_file, read_input},
};

pub struct InitCommand;

impl cmd::Command for InitCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let ip_addr = read_input("Enter IP address: ")?;
        // unsafe {
        //     env::set_var("PASSPORT_IP", &ip_addr);
        // }
        // assert_eq!(env::var("PASSPORT_IP"), Ok(ip_addr.to_string()));
        write_file("PASSPORT_IP", &ip_addr)?;
        cprintln!("<green>Now using:</> {}", &ip_addr);
        Ok(())
    }
}
