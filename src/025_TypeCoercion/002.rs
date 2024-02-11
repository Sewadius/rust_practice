// Allow overflowing
#[allow(overflowing_literals)]
fn main() {
    assert_eq!(u8::MAX, 255);

    let v: u8 = 1000 as u8;
    println!("v = {}", v);

    println!("Success!");
}