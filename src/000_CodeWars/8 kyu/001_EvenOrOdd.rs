use std::io;
use std::io::Write;
use std::io::stdin;

// Even or Odd
#[macro_use]
mod macros {
    #[macro_use]
    mod custom_io;
}

// The program determines the parity of an i32 number
fn main() {
    println!("The program determines the parity of an i32 number.\n");

    print!("Enter the integer number: ");
    flush_stdout!();

    let number: i32 = get_i32_value!();
    let result: &str = even_or_odd(number);

    println!("The result is: {}", result);
    wait_for_key_press!();
}

/// Determine even or odd number
fn even_or_odd(number: i32) -> &'static str {
    match number & 1 == 0 {
        true => "Even",
        false => "Odd"
    }
}
