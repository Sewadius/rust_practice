use std::io;

// HQ9+ - 900
fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Error");
    input = input.trim().to_string();

    let mut check: bool = false;
    for ch in ["H", "Q", "9"] {
        if input.contains(ch) {
            check = true;
            break;
        }
    }

    if check { println!("YES") } else { println!("NO") }
}
