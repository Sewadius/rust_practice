// Unimportant variables
fn main() {
    let (x, y);

    (x, ..) = (3, 4);   // Change x
    [.., y] = [1, 2];   // Change y

    assert_eq!([x, y], [3, 2]);
    println!("Success!");
}
