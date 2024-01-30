// Option<T> example
fn main() {
    let mut five = Some(5);
    println!("Initial value: {:?}", five);
    
    five = plus_one(five);
    println!("Result: {:?}", five);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}
