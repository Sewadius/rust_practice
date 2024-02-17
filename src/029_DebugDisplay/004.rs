#[derive(Debug)]
#[allow(dead_code)]
struct Person {
    name: String,
    age: u8
}

// Using {:#?} for better print
fn main() {
    let person: Person = Person { name: "Sunface".to_string(), age: 18 };
    println!("{:#?}", person);
}
