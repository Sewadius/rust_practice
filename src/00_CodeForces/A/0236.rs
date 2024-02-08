use std::{collections::HashSet, io};

// A. Девушка или Юноша
fn main() {
    const GIRL: &str = "CHAT WITH HER!";
    const BOY: &str = "IGNORE HIM!";

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Error");
    input = input.trim().to_string();

    let count_chars: usize = count_unique_chars(&input);

    if count_chars % 2 == 0 { println!("{}", GIRL) } else { println!("{}", BOY) }
}

fn count_unique_chars(s: &str) -> usize {
    let mut unique_chars: HashSet<char> = HashSet::new();

    for ch in s.chars() {
        unique_chars.insert(ch);
    }

    unique_chars.len()
}
