use std::io::{self, Write};

// Powers of 2
fn main() {
    println!("The program takes a non-negative integer and returns a list of all powers of 2.\n");

    print!("Enter the n: ");
    io::stdout().flush().unwrap();

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let number: u8 = input.trim().parse().unwrap();
    let result: Vec<u128> = powers_of_two(number);

    println!("The result is: {:?}", result);
    wait_key_pressed();
}

fn powers_of_two(n: u8) -> Vec<u128> {
    (0..=n).map(|e| 2_u128.pow(e as u32)).collect()
}

fn wait_key_pressed() {
    println!("\nPress any key to continue...");
    io::stdin().read_line(&mut String::new()).unwrap();
}
