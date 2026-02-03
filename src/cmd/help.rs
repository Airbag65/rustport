use color_print::cprintln;

use crate::{cmd::Command, utilities::file::read_file};

pub struct HelpCommand;

impl Command for HelpCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let title = read_file("rustport_title.txt")?;
        cprintln!("<rgb(211, 69, 22)>{}</>", title);
        println!("More help incomming");
        Ok(())
    }
}
