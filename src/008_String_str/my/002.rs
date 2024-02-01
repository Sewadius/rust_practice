#[macro_use]
mod macros {
    #[macro_use]
    mod custom_io;
}

// The program receives a string and outputs it (with macros)
fn main() {
    println!("The program receives a string and outputs it (with macros).\n");

    print!("Enter a string: ");
    flush_stdout!();

    let mut input = String::new();
    get_string_value!(input);

    println!("You entered: {}", input.trim());
    wait_for_key_press!();
}
