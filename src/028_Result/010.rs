use std::num::ParseIntError;

// Using Result in main function (second variant "?")
fn main() -> Result<(), ParseIntError> {
    let number_str: &str = "10";
    let number: i32 = number_str.parse()?;
    println!("{}", number);
    Ok(())
}
