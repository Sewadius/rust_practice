// Returning the string slice
fn main() {
    let x: &str = invalid_output();
    println!("{}", x);
 }
 
 fn invalid_output() -> &'static str {
     "foo"
 }
 