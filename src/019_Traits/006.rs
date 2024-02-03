// Using trait objects
struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> String;
}

impl Animal for Sheep {
    fn noise(&self) -> String {
        "baaaah!".to_string()
    }
}

impl Animal for Cow {
    fn noise(&self) -> String {
        "moooo!".to_string()
    }
}

fn main() {
    let random_number: f64 = 0.234;
    let animal: Box<dyn Animal> = random_animal(random_number);

    println!("You've randomly chosen an animal, and it says {}", animal.noise());
}

fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        return Box::new(Sheep {})
    }
    Box::new(Cow {})
}
