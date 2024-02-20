// Returning not the reference
fn main() {
    let x: String = invalid_output();
    println!("{}", x);
 }
 
 fn invalid_output() -> String {
     String::from("foo")
 }
 