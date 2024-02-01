// Using generic in structs
#[derive(Debug)]
#[allow(dead_code)]
struct Point<T, U> {
    x: T,
    y: U
}

fn main() {
    let p: Point<i32, String> = Point {x: 5, y: "hello".to_string()};

    println!("{:?}", p);
    println!("Success!");
}
