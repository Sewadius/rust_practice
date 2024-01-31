// Careful with mutable references in match
fn main() {
    let mut v: String = String::from("hello,");
    let r = &mut v;

    match r {
        value => value.push_str(" world")
    }

    println!("{}", v);
}
