use std::io;

// A. Слишком длинные слова - 800
fn main() {
    let mut result: Vec<String> = Vec::new();

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    let mut words: i32 = input.trim().parse().unwrap();

    while words > 0 {
        let mut word: String = String::new();
        io::stdin().read_line(&mut word).expect("Failed to read");
        word = word.trim().to_string();

        if word.len() > 10 {
            let mut word_change: String = String::new();
            let length: usize = word.len();

            word_change.push_str(&word[..1]);
            word_change.push_str(&(length - 2).to_string());
            word_change.push_str(&word[length - 1..]);

            result.push(word_change);
        } else {
            result.push(word);
        }
        words -= 1;
    }
    print_result(result);
}

fn print_result(result: Vec<String>) {
    for element in result {
        println!("{}", element);
    }
}
