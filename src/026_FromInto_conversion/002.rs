// Using for String
fn main() {
    let my_str: &str = "hello";

    let string1: String = String::from(my_str);
    let string2: String = my_str.to_string();
    let string3: String = my_str.into();

    assert_eq!(string1, string2);
    assert_eq!(string1, string3);

    println!("Success!");
}
