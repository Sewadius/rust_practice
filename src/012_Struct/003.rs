// Mutable struct for changing fields
struct Person {
    name: String,
    age: u8
}

fn main() {
    let age: u8 = 18;
    let mut p = Person {
        name: String::from("sunface"),
        age
    };

    p.age = 30;
    p.name = String::from("sunfei");

    println!("Success!");
}
