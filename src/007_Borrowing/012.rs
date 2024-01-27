// Borrow a mutable object as immutable
fn main() {
    let mut s: String = String::from("hello, ");
    borrow_object(&s);

    s.push_str("world");
    
    println!("{s}");
    println!("Success!");
}

fn borrow_object(s: &String) {
    println!("{s}");
}
