use std::io;

// A. Математика спешит на помощь - 800
fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Error");

    let mut parts: Vec<&str> = input.trim().split('+').collect();
    parts.sort();

    println!("{}", parts.join("+"));
}
