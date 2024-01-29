// Create a struct
struct Person {
    name: String,
    age: u8,
    hobby: String
}

fn main() {
    let age = 30;
    let _p = Person {
        name: String::from("sunface"),
        age,
        hobby: String::from("coding")
    };

    print!("Success!");
}
