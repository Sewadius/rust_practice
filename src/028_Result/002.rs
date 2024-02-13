use std::num::ParseIntError;

// Using "?" in main function
fn main() -> Result<(), ParseIntError> {
    let number_str: &str = "10";
    let number = number_str.parse::<i32>()?;
    println!("{}", number);
    Ok(())
}
