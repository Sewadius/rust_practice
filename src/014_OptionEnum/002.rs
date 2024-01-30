// Unpack Option<T> value for Some()
fn main() {
    let five = Some(5);
    let six: Option<i32> = plus_one(five);

    if let Some(n) = six {
        println!("Result: {}", n);
        println!("Success!");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}
