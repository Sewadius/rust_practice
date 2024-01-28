// push(), push_str() and += &str
fn main() {
    let mut s: String = String::from("Hello");
    s.push(',');
    s.push_str(" world");
    s += "!";

    print!("{s}");
}
