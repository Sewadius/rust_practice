// Int pattern in match
fn main() {
    for i in 1..15 {
        match_number(i);
    }
}

fn match_number(n: i32) {
    match n {
        1 => println!("One!"),
        2 | 3 | 4 | 5 => println!("match 2 -> 5"),
        6..=10 => println!("match 6 -> 10"),
        _ => println!("match -inf -> 0 or 11 -> +inf")
    };
}
