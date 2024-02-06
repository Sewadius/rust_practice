use std::io;

// Convert a Number to a String
fn main() {
    println!("The program converts i32 number to String object.\n");

    let number: i32 = 123;
    let string: String = number_to_string(number);

    println!("For i32 number \"{}\": {}", number, type_of(&number));
    println!("For String object \"{}\": {}", string, type_of(&string));

    println!("\nPress any key to continue...");
    io::stdin().read_line(&mut String::new()).expect("Failed");
}

fn number_to_string(i: i32) -> String {
    i.to_string()
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
