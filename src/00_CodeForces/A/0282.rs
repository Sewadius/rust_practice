use std::io;

// A. Bit++ - 800
fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Error");
    let sentences: i32 = input.trim().parse::<i32>().unwrap();
    
    process_input(sentences);
}

fn process_input(sentences: i32) {
    let mut result: i32 = 0;

    for _ in 0..sentences {
        let mut sentence: String = String::new();
        io::stdin().read_line(&mut sentence).expect("Error");

        if sentence.contains("++") { result += 1} else { result -= 1}
    }

    println!("{}", result);
}
