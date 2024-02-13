use std::io;

// if an element is the word "flick", switch to the opposite bool value
fn main() {
    println!("The program always returns True/true for every item in a given list.");
    println!("However, if an element is the word 'flick', switch to the opposite bool value.\n");

    let words: [&str; 6] = ["codewars", "flick", "code", "wars", "flick", "pet"];
    let result: Vec<bool> = flick_switch(&words);

    println!("{:?}", words);
    println!("{:?}", result);

    println!("\nPress any key to continue...");
    io::stdin().read_line(&mut String::new()).unwrap();
}

fn flick_switch(list: &[&str]) -> Vec<bool> {
    let mut flag: bool = true;
    let mut vec: Vec<bool> = Vec::new();

    for &word in list {
        if word == "flick" { flag = !flag; }
        vec.push(flag);
    }

    vec
}
