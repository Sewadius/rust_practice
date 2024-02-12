use core::num;
use std::{fs, io};

// Custom error handling
enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for CliError {
    fn from(value: io::Error) -> Self {
        CliError::IoError(value)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(value: num::ParseIntError) -> Self {
        CliError::ParseError(value)
    }
}

fn main() {
    println!("Success!");
}

#[allow(dead_code)]
fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    let contents: String = fs::read_to_string(&file_name)?;
    let num: i32 = contents.trim().parse()?;
    Ok(num)
}
