use std::io;

// А. Арбуз - 800
fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");

    let weight: i32 = match input.trim().parse::<i32>() {
        Ok(input) => input,
        Err(_) => 0
    };

    match check_weight(weight) {
        true => println!("YES"),
        false => println!("NO")
    }
}

fn check_weight(weight: i32) -> bool {
    weight > 2 && weight % 2 == 0
}
