// Borrow an immutable object as immutable
fn main() {
    let s = String::from("hello, ");
    borrow_object(&s);
    println!("Success!");
}

fn borrow_object(_s: &String) {}
