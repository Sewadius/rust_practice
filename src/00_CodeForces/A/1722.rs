use std::io;

// A. Проверка правописания - 800
fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let num_words: usize = input.trim().parse().unwrap();
    let mut name: Vec<char> = "Timur".chars().collect();
    name.sort();

    for _ in 0..num_words {
        input.clear();
        io::stdin().read_line(&mut String::new()).unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let word: &str = input.trim();
        let mut sorted_word: Vec<char> = word.chars().collect();
        sorted_word.sort();

        match sorted_word == name  {
            true => println!("YES"),
            false => println!("NO"),
        };
    }
}
