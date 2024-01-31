#[macro_use]
mod macros {
    #[macro_use]
    mod custom_io;
    #[macro_use]
    mod custom_string;
}

// Get a string from user and reverse it - ver.2 (with macros)
fn main() {
    print!("Enter initial string: ");
    flush_stdout!();

    let mut input: String = String::new();
    get_string_value!(&mut input);

    let reversed: String = reverse_string!(&input.trim());
    println!("The reversed string: {}", reversed);

    wait_for_key_press!();
}
