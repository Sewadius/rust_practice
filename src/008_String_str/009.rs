// Convers String -> &str
fn main() {
    let s: String = "hello, world".to_string();
    let _s1: &str = &s;     // &String -> &str

    println!("Success!");
}
