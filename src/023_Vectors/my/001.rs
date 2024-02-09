use std::io::{self, Write};

// The program fills and outputs items for Vec<String>
fn main() {
    println!("The program fills and outputs items for Vec<String>.\n");
    print_welcome();

    let mut input: String = get_input();
    let mut counter: usize;
    
    // Getting total number of elements
    loop {
        counter = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Failed to parse to usize!");
                print_welcome();
                input = get_input();
                continue;    
            }
        };
        break;
    }

    // Checks for zero elements input
    if counter == 0 {
        println!("There are no elements for adding to the vector!");
        wait_press_key();
        return;
    }

    println!();

    let mut v: Vec<String> = Vec::with_capacity(counter);

    for i in 0..counter {
        print!("Enter the {} element: ", i + 1);
        io::stdout().flush().expect("Error");
        v.push(get_input());
    }

    println!("\nThe vector's size is: {}", v.len());
    println!("The vector's capacity is: {}\n", v.capacity());

    while let Some(item) = v.pop() {
        println!("The {} element is: {}", counter, item);
        counter -= 1;
    }

    wait_press_key();

}

/// Prints welcome message
fn print_welcome() {
    print!("Enter the number of elements for adding to the vector: ");
    io::stdout().flush().expect("Error");
}

/// Get input string from user
fn get_input() -> String {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Error");
    input.trim().to_string()
}

fn wait_press_key() {
    println!("\nPress any key to continue...");
    io::stdin().read_line(&mut String::new()).expect("Error");
}