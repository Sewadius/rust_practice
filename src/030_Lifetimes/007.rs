// Using lifetime in function
fn main() {
    let s: String = String::from("foo");
    let x: &str = invalid_output(&s);

    println!("{}", x);
}

fn invalid_output<'a>(s: &'a str) -> &'a str { s } 
