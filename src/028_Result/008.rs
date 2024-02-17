use std::num::ParseIntError;

#[allow(non_camel_case_types)]
type Res<i32> = Result<i32, ParseIntError>;

// Using own alias for long types
fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}

fn multiply(first_number_str: &str, second_number_str: &str) -> Res<i32> {
    first_number_str.parse::<i32>()
        .and_then(|first_number| {
            second_number_str.parse::<i32>()
                .map(|second_number| first_number * second_number)
        })
}

fn print(result: Res<i32>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}
