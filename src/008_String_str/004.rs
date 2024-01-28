// Mutable String type
fn main() {
    let mut s: String = String::new();
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");
    println!("{s}");
    println!("Success!");
}
