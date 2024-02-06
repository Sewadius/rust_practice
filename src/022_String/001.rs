// push_str() & push() methods
fn main() {
    let mut s: String = String::from("hello, ");
    s.push_str("world");
    s.push('!');

    move_ownership(s.clone());

    assert_eq!(s, "hello, world!");
    println!("Success!");
}

fn move_ownership(s: String) {
    println!("Ownership of \"{}\" is moved here!", s);
}
