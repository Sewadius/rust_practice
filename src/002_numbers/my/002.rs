use std::io::{self, Write, stdin};

#[macro_use]
mod macros {
    #[macro_use]
    mod custom_io;
}

// The program reads two i32 numbers and outputs their sum (with macros)
fn main() {
    println!("The program reads two i32 numbers and outputs their sum (with macros).\n");

    print!("Enter first number: ");
    flush_stdout!();
    let number_1: i32 = get_i32_value!();

    print!("Enter second number: ");
    flush_stdout!();
    let number_2: i32 = get_i32_value!();

    println!("Sum of numbers: {}", sum_count(number_1, number_2));

    wait_for_key_press!();
}

/// Sum for two numbers
fn sum_count(a: i32, b: i32) -> i32 {
    a + b
}
