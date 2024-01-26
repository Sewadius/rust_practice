// Change or not change for String
fn main() {
    let s = give_ownership();
    println!("{}", s);
}

fn give_ownership() -> String {
    let s: String = String::from("hello, world");
    //let _s = s.into_bytes();          // Change the String 
    let _s = s.as_bytes();              // Don't change the String
    s
}
