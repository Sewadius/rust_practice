// Get two i32 vars and counting for additon/substraction
use std::io::{stdin, stdout, Write};

fn main() {
    let mut input: String = String::new();
    let (number1, number2);

    // Get number 1
    print!("Enter the number 1: ");
    number1 = get_i32_value(&mut input);
    input.clear();

    // Get number 2
    print!("Enter the number 2: ");
    number2 = get_i32_value(&mut input);

    // Printing the result
    println!("\nNumber 1 = {}; Number 2 = {}\n", number1, number2);
    print_operations_result(number1, number2);
    

    // Waiting a key press
    println!("\nPress any key to continue...");
    let _ = stdin().read_line(&mut input);
}

/// Get int from user's input 
fn get_i32_value(input: &mut String) -> i32 {
    stdout().flush().expect("Failed to flush stdout");
    stdin().read_line(input).expect("Not a valid string");
    input.trim().parse().expect("Not a valid number")
}

/// Prints the resulf of operations
fn print_operations_result(n1: i32, n2: i32) {
    if n2 >= 0 {
        println!("Addition for ({} + {}) = {}", n1, n2, n1 + n2);
        println!("Subtraction for ({} - {}) = {}", n1, n2, n1 - n2);
    } else {
        println!("Addition for ({} - {}) = {}", n1, -n2, n1 + n2);
        println!("Subtraction for ({} + {}) = {}", n1, -n2, n1 - n2);
    }
}
