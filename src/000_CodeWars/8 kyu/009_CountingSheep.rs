use std::io::{self, Write};
use rand::Rng;

// Counting sheep
fn main() {
    println!("The program counts the number of sheeps in the array.\n");

    print!("Enter the length of array: ");
    io::stdout().flush().unwrap();

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let length: usize = input.trim().parse().unwrap();
    let sheeps: Vec<bool> = 
        (0..length).map(|_| rand::thread_rng().gen_bool(0.5)).collect();
    
    println!("Initial array is: {:?}", sheeps);

    let result: u8 = count_sheep(&sheeps);
    println!("The result is: {}", result);

    wait_key_pressed();
}

fn count_sheep(sheep: &[bool]) -> u8 {
    sheep.iter().filter(|&&el| el).count() as u8
}

fn wait_key_pressed() {
    println!("\nPress any key to continue...");
    io::stdin().read_line(&mut String::new()).unwrap();
}
