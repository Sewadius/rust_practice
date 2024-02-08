use std::io::{self, Write};

// The program makes the last character in the string capitalized
fn main() {
    println!("The program makes the last character in the string capitalized.\n");
    print!("Enter the string: ");
    io::stdout().flush().expect("Error");

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Error");
    input = input.trim().to_string();

    println!("Result is: {}", capitalize_last(&input));

    println!("\nPress any key to continue...");
    io::stdin().read_line(&mut String::new()).expect("Error");
}

/// Capitalizing the last char in the given string
fn capitalize_last(s: &str) -> String {
    let mut c = s.chars();
    let mut result: String = String::new();

    while let Some(ch) = c.next() {
        if c.clone().count() == 0 {
            result.push_str(&ch.to_uppercase().to_string());
        } else {
            result.push(ch);
        }
    }
    result
}
