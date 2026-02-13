use crate::{
    cmd::Command,
    utilities::{generate_password, print_boxed},
};

#[allow(unused)]
pub struct GenerateCommand;

impl Command for GenerateCommand {
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Generated password:");
        print_boxed(&generate_password());
        Ok(())
    }
}
