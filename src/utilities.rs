use std::{
    fmt::Display,
    io::{self, Write},
};

pub mod file;

pub fn read_input(prompt: impl Display) -> Result<String, Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    print!("{prompt}");
    let _ = std::io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer)?;
    buffer = buffer.trim_ascii_end().to_owned();
    Ok(buffer)
}
