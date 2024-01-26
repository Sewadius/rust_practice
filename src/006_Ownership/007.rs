// Struct example
fn main() {
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person: Person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    let Person {ref name, ref age} = person;

    println!("The person's age is {}", age);
    println!("The person's name is {}", name);

    println!("From person object, age {}", person.age);
    println!("From person object, name {}", person.name);
}
