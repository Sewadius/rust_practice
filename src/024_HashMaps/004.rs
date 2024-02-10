use std::collections::HashMap;

// Derive for custom structs
#[derive(Debug, Hash, PartialEq, Eq)]
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    fn new(name: &str, country: &str) -> Self {
        Viking {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}

fn main() {
    let vikings = HashMap::from([
        (Viking::new("Einar", "Norway"), 25),
        (Viking::new("Olaf", "Denmark"), 24),
        (Viking::new("Harald", "Iceland"), 12),
    ]);

    for (viking, health) in &vikings {
        println!("{:?} has {} hp", viking, health);
    }
}
