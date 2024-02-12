use std::io;

// A. Халк - 800
fn main() {
    let mut phrase: String = String::from("I hate ");

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Error to read line");

    let level: u8 = input.trim().parse::<u8>().unwrap();

    for i in 1..level {
        if i % 2 == 0 {
            phrase.push_str("that I hate ");
        } else {
            phrase.push_str("that I love ");
        }
    }

    println!("{}", phrase + "it");
}
