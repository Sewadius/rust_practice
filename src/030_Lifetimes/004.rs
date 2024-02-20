// Using lifefime in function with &str
fn main() {
    let x: &str = "long";
    let y: &str = "longest";

    println!("{}", longest(x, y));
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { return x; }
    y
}
