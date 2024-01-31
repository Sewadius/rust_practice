#[macro_use]
mod macros {
    #[macro_use]
    mod custom_io;
}

// Get a string from user and prints in out (with macros)
fn main() {
    print!("Enter a string: ");
    flush_stdout!();

    let mut input = String::new();
    get_string_value!(input);

    println!("You entered: {}", input.trim());
    wait_for_key_press!();
}
