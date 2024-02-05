use std::collections::HashMap;

#[macro_use]
mod macros {
    #[macro_use]
    mod custom_io;
}

// The program counts the frequency of each word in the string
fn main() {
    println!("The program counts the frequency of each word in the string.\n");
    print!("Enter a sentence: ");
    flush_stdout!();

    let mut input: String = String::new();
    
    get_string_value!(&mut input);
    let frequencies: HashMap<String, usize> = word_frequency(&input);
    println!("The result is: {:?}", frequencies);

    wait_for_key_press!();
}

/// Create a HashMap with word frequency in a sentence
fn word_frequency(input: &String) -> HashMap<String, usize> {
    let mut result: HashMap<String, usize> = HashMap::new();

    for word in input.split_whitespace() {
        *result.entry(word.to_string()).or_insert(0) += 1;
    }
    result
}
