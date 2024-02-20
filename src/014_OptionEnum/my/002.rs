use std::io::{self, Write};

// Using Option<f64> for handle with divide
fn main() {
    println!("The program shows handling with divide using Option<f64>.\n");

    print!("Enter first float number: ");
    io::stdout().flush().unwrap();

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let number_1: f64 = input.trim().parse().unwrap();

    print!("Enter second float number: ");
    io::stdout().flush().unwrap();
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();

    let number_2: f64 = input.trim().parse().unwrap();

    let result: Option<f64> = divide(number_1, number_2);
    handle_result(result);

    wait_key_pressed();
}

fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    Some(numerator / denominator).filter(|_| denominator != 0.0)
}

fn handle_result(result: Option<f64>) {
    match result {
        Some(value) => println!("Result is: {}", value),
        None => println!("Cannot divide by 0"),
    }
}

fn wait_key_pressed() {
    println!("\nPress any key to continue...");
    io::stdin().read_line(&mut String::new()).unwrap();
}
