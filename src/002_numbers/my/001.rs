use std::io::{self, Write};

// The program receives an i32 number and determines even/oddness
fn main() {
    println!("The program receives an i32 number and determines even/oddness.\n");

    print!("Enter the integer number: ");
    let _ = io::stdout().flush();

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read string");

    let number: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Failed parsing to integer!");
            wait_any_key_pressed();
            return;
        }
    };

    println!("Result: {}", even_or_odd(number));
    wait_any_key_pressed();
}

/// Function for check even / odd
fn even_or_odd(number: i32) -> &'static str {
    match number % 2 {
        0 => "Even number",
        _ => "Odd number"
    }
}

/// Waiting for key pressed
fn wait_any_key_pressed() {
    println!("\nPress any key to continue...");
    let _ = io::stdin().read_line(&mut String::new());
}
