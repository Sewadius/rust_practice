// The program finds the maximum of two integers
use std::io;
use std::io::Write;
use std::io::stdin;

#[macro_use]
mod macros {
    #[macro_use]
    mod custom_io;
}

fn main() {
    println!("The program finds the maximum of two integers.\n");
    print!("Enter the first number: ");
    flush_stdout!();

    let number_1: i32 = get_i32_value!();
    
    print!("Enter the second number: ");
    flush_stdout!();

    let number_2: i32 = get_i32_value!();

    println!("The maximum is: {}", find_max(number_1, number_2));
    wait_for_key_press!();
}

fn find_max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}
