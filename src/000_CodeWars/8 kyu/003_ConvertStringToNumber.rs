use std::io;

// Convert a String to a Number
fn main() {
    println!("The program converts &str to i32 number.\n");

    let s: &str = "1025";
    let number: i32 = string_to_number(s);

    println!("For &str value: \"{}\": {}", s, type_of(&s));
    println!("For i32 value: \"{}\": {}", number, type_of(&number));

    println!("\nPress any key to continue...");
    io::stdin().read_line(&mut String::new()).expect("Error");

}

fn string_to_number(s: &str) -> i32 {
    s.parse().unwrap()
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
