use std::num::ParseIntError;

// Examle for using ParseIntError
fn main() {
    let result: Result<i32, ParseIntError> = multiply("10", "2");
    assert_eq!(result, Ok(20));

    let result: Result<i32, ParseIntError> = multiply("2", "4");
    assert_eq!(result.unwrap(), 8);

    println!("Success!");
}

fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1: i32 = n1_str.parse().unwrap();
    let n2: i32 = n2_str.parse().unwrap();
    Ok(n1 * n2)
}
