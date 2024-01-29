// Get string from user and prints the reversed string
use std::io::{stdin, stdout, Write};

const MSG: &str = "Failed to flush stdout";

fn main() {
    let mut input: String = String::new();

    print!("Print a string: ");
    get_string_value(&mut input);

    let reversed_string: String = reverse_string(&mut input);

    // Result output
    println!("{}", &reversed_string);

    // Waiting a key press
    println!("\nPress any key to continue...");
    let _ = stdin().read_line(&mut input);
}

/// Get string input from user
fn get_string_value(input: &mut String) -> String {
    stdout().flush().expect(MSG);
    stdin().read_line(input).expect("Not a valid string");
    input.trim().to_string()
}

/// Function for reverse string
fn reverse_string(s: &mut String) -> String {
    s.pop();    // Remove "\n" from input
    let mut chars: Vec<char> = s.chars().collect();
    chars.reverse();
    chars.into_iter().collect()
}