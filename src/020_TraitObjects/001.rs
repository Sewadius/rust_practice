// Trait object example
trait Animal {
    fn say_hi(&self) -> String;
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn say_hi(&self) -> String {
        "Gaff!".to_string()
    }
}
impl Animal for Cat {
    fn say_hi(&self) -> String {
        "Myyu!".to_string()
    }
}

fn main() {
    let animal1: &dyn Animal = return_animal("cat");
    let animal2: &dyn Animal = return_animal("dog");

    println!("{}", animal1.say_hi());
    println!("{}", animal2.say_hi());
}

fn return_animal(s: &str) -> &dyn Animal {
    match s {
        "dog" => &Dog {},
        "cat" => &Cat {},
        _ => panic!()
    }
}
