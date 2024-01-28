// Get slices with indexes
fn main() {
    let s1: String = String::from("hi, world");
    let h: &str = &s1[0..1];    // "h"

    assert_eq!(h, "h");
    println!("Success!");
}
