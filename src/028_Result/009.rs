use std::num::ParseIntError;

// Using Result in main function
fn main() -> Result<(), ParseIntError> {
    let number_str: &str = "10";
    let number: i32 = match number_str.parse() {
        Ok(number) => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);
    Ok(())
}
