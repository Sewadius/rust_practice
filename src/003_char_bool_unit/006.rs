// Size of unit type
use std::mem::size_of_val;

fn main() {
    let unit: () = ();

    println!("Size of unit () = {}", size_of_val(&unit));
    assert!(size_of_val(&unit) == 0);

    print!("Success!");
}
