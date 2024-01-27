// Borrow from the reference
fn main() {
    let a: String = String::from("hello");
    borrow_object(&a);

    println!("Success!");
}

fn borrow_object(_s: &String) {}
