// Owner for the part of tuple
fn main() {
    let t: (String, String) = (String::from("hello"), String::from("world"));
    let _s = t.0;

    println!("{:?}", t.1); // Because "t" is not the owner for "t.0"
}
