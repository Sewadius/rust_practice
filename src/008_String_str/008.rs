// Two ways to convers &str -> String
fn main() {
    let s: &str = "hello, world";
    greetings(s.to_string());       // &str -> String
    greetings(String::from(s));     // Second way
}

fn greetings(s: String) {
    println!("{s}");
}
