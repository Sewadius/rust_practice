#[macro_use]
mod macros {
    #[macro_use]
    mod custom_io;
    #[macro_use]
    mod custom_string;
}

// The program receives the string and outputs the reversed string (with macros)
fn main() {
    println!("The program receives the string and outputs the reversed string (with macros).\n");

    print!("Enter initial string: ");
    flush_stdout!();

    let mut input: String = String::new();
    get_string_value!(&mut input);

    let reversed: String = reverse_string!(&input.trim());
    println!("The reversed string: {}", reversed);

    wait_for_key_press!();
}
