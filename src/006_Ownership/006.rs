// Box as smarter pointer
fn main() {
    let x = Box::new(5);
    let mut y = Box::new(1);

    *y = 4;
    assert_eq!(*x, 5);
    assert_eq!(*y, 4);
    println!("Success!");
}
