// Size of chars
use std::mem::size_of_val;

fn main() {
    let c1 = 'a';
    println!("Size of c1 (char): {}", size_of_val(&c1));
    assert_eq!(size_of_val(&c1), 4);    // 4 bytes

    let c2: char = '`';
    assert_eq!(size_of_val(&c2), 4);

    println!("Success!");
}
