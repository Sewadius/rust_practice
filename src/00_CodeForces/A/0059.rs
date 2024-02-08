use std::io;

// A. Слово - 800
fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Error");
    input = input.trim().to_string();

    println!("{}", process_word(&input));
}

fn process_word(s: &str) -> String {
    let mut c = s.chars();
    let mut counter: u8 = 0;

    while let Some(ch) = c.next() {
        if ch == ch.to_ascii_uppercase() {
            counter += 1;
        }
    }

    match counter > (s.len() / 2) as u8 {
        true => string_change_case(s, true),
        false => string_change_case(s, false)
    }
}

fn string_change_case(s: &str, upper: bool) -> String {
    let mut result: String = String::new();
    let mut c = s.chars();

    while let Some(ch) = c.next() {
        if upper {
            result.push_str(&ch.to_uppercase().to_string());
        } else {
            result.push_str(&ch.to_lowercase().to_string());
        }
    }
    result
}
