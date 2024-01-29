// Short syntax for create struct
struct Person {
    name: String,
    age: u8
}

fn main() {
    let p: Person = build_person(String::from("Nick"), 16);

    println!("Name {}, age {}", p.name, p.age);
}

fn build_person(name: String, age: u8) -> Person {
    Person {name, age}
}
