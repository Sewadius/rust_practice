use std::io;

// A. Капитализация слова - 800
fn main() {
    let mut word: String = String::new();
    io::stdin().read_line(&mut word).expect("Error");
    word = word.trim().to_string();

    let result: String = capitalize(&word);
    println!("{}", result);
}

fn capitalize(s: &str) -> String {
    let mut c: std::str::Chars<'_> = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str()
    }
}
