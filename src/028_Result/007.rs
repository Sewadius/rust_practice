use std::num::ParseIntError;

// Two different ways with functions
fn main() {
    assert_eq!(Ok(10), multiply("2", "5"));

    let twenty = multiply1("10", "2");
    println!("{}", twenty.unwrap());
    println!("Success!");
}

fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    match n1_str.parse::<i32>() {
        Ok(n1) => {
            match n2_str.parse::<i32>() {
                Ok(n2) => Ok(n1 * n2),
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}

fn multiply1(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    n1_str
        .parse::<i32>()
        .and_then(|n1| n2_str.parse::<i32>()
        .map(|n2| n1 * n2))
}
