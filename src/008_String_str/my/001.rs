use std::io::{stdin, stdout, Write};

const MSG: &str = "Failed to flush stdout";

// The program receives the string and outputs the reversed string
fn main() {
    println!("The program receives the string and outputs the reversed string.\n");

    let mut input = String::new();

    print!("Print a string: ");
    stdout().flush().expect(MSG);
    get_string_value(&mut input);

    let reversed_string = reverse_string(&mut input.trim());

    // Result output on the same line
    println!("Reversed: {}", reversed_string);

    // Waiting for a key press
    println!("\nPress Enter to continue...");
    let _ = stdin().read_line(&mut input);
}

/// Get string input from user
fn get_string_value(input: &mut String) {
    stdin().read_line(input).expect("Not a valid string");
}

/// Function for reverse string
fn reverse_string(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.reverse();
    chars.into_iter().collect()
}
