// Sum function
fn main() {
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);
    print!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
