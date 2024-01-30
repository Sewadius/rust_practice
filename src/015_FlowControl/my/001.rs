use std::io::{self, Write};

const MSG: &str = "Enter the positive integer number: ";
const FAIL: &str = "Failed to read string";
const INVALID: &str = "Invalid number! Try again!";

const THREE: i32 = 3;

// Get int from user and write (n * 3) times, loop example
fn main() {
    print!("{}", MSG);
    let _ = io::stdout().flush();

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect(FAIL);

    let number: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", INVALID);
            return;
        }
    };

    if handle_with_number(number) {
        println!("The number should be more than zero!");
        return;
    }

    println!("\nPress any key to continue...");
    wait_for_key_press();
}

/// Handle with integer number
fn handle_with_number(number_of_multiples: i32) -> bool {
    if number_of_multiples <= 0 {
        return true;
    }
    for i in 1..=number_of_multiples {
        print!("{} ", i * THREE);
    }
    println!();
    false
}

/// Wait for user to press any key
fn wait_for_key_press() {
    let _ = io::stdin().read_line(&mut String::new());
}