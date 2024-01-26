// Many references
fn main() {
    let mut s: String = String::from("hello");

    let r1 = &s;
    let r2: &String = &s;
    println!("{} and {}", r1, r2);

    // Only one mutable reference
    let r3 = &mut s;
    println!("{}", r3);
}
