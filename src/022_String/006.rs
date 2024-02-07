// String::with_capacity() example
fn main() {
    let mut s: String = String::with_capacity(25);

    println!("String's capacity: {}", s.capacity());

    for _ in 0..2 {
        s.push_str("hello");
        println!("String's capacity: {}", s.capacity());
    }
}
