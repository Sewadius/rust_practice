// Using trait bounds
fn main() {
    assert_eq!(sum(1, 2), 3);
    println!("Success!");
}

fn sum<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
    x + y
}
