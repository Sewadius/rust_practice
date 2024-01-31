// At operator @ for checking values
struct Point {
    x: i32,
    y: i32
}
fn main() {
    let p: Point = Point {x: 1, y: 20};

    match p {
        Point {x, y: 0} => println!("On the x axis at {}", x),
        Point {x: 0..=5, y: y @ (10 | 20 | 30)} => println!("On the y axis at {}", y),
        Point {x, y} => println!("On neither axis: ({}, {})", x, y)
    };
}
