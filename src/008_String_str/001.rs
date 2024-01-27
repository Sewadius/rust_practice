// &str slices from String object
fn main() {
    let s: String = String::from("hello world!");

    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];

    println!("{hello}");    // "hello"
    println!("{world}");    // "world"
}
