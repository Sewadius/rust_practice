use std::io;

// A. Укладка доминошками - 800
fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Error");

    let values: Vec<&str> = input.trim().split(' ').collect();

    let mut result: i32 = 1;

    for el in values {
        let value: i32 = el.parse().unwrap();
        result *= value
    }

    println!("{}", result / 2);
}
