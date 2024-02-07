// Using utf8_slice crate
use utf8_slice;

fn main() {
    let s: &str = "The rocket goes to the moon!";
    let rocket: &str = utf8_slice::slice(s, 4, 10);

    println!("{}", rocket);
    println!("Success!");
}
