// Variable initialize
fn main() {
    let x: i32 = 5;
    let mut y: i32 = 1;
    y += 2;

    assert_eq!(x, 5);
    assert_eq!(y, 3);
    println!("Success!");
}
