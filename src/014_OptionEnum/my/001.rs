use std::io;

// Example of using Option<i32> with and without value
fn main() {
    println!("Example of using Option<i32> with and without value.\n");

    let no_value: Option<i32> = None;
    let some_value: Option<i32> = Some(42);

    check_value(no_value);
    check_value(some_value);

    println!("\nPress any key to continue...");
    let _ = io::stdin().read_line(&mut String::new());
}

fn check_value(value: Option<i32>) {
    match value {
        Some(value) => println!("The value is {}", value),
        None => println!("No value provided!")
    }
}
