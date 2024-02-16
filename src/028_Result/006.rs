use std::num::ParseIntError;

// map && and_then usuing
fn main() {
    assert_eq!(add_two_map("4").unwrap(), 6);
    assert_eq!(add_two_then("4").unwrap(), 6);

    println!("Success!");
}

fn add_two_map(n_str: &str) -> Result<i32, ParseIntError> {
    n_str.parse::<i32>().map(|n| n + 2)
}

fn add_two_then(n_str: &str) -> Result<i32, ParseIntError> {
    n_str.parse::<i32>().and_then(|n| Ok(n + 2))
}
