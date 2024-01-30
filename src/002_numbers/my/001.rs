use std::io::{self, Write};

// Get user's input and prints Even or Odd number
fn main() {
    print!("Enter the integer number: ");
    let _ = io::stdout().flush();

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read string");

    let number: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invlid input! Please enter a valid integer.");
            return;
        }
    };

    println!("Result: {}", even_or_odd(number));
    println!("\nPress any key to continue...");
    
    let _ = io::stdin().read_line(&mut String::new());
}

/// Function for check even / odd
fn even_or_odd(number: i32) -> &'static str {
    match number % 2 {
        0 => "Even number",
        _ => "Odd number"
    }
}
