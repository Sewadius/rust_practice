// Assert types & type_of()
use std::any::type_name;

fn main() {
    let x: i32 = 5;

    println!("{}", "u32".to_string());
    println!("{}", type_of(&x));
    assert_eq!("&i32".to_string(), type_of(&x));
    println!("Success!");
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
