use std::num::ParseIntError;

// Using "?" in function to get value
fn main() {
    assert_eq!(multiply("3", "4").unwrap(), 12);
    println!("Success!");
}

fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1 = n1_str.parse::<i32>()?;
    let n2 = n2_str.parse::<i32>()?;
    Ok(n1 * n2)
}