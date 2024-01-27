// Using Box for boxing str
fn main() {
    let s: Box<str> = "Hello, world".into();
    greetings(&s);
}

fn greetings(s: &str) {
    println!("{}", s);
}
