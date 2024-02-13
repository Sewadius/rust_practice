use core::fmt;

// Display for custom types
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The point is ({}, {})", self.x, self.y)
    }
}

fn main() {
    let origin: Point = Point { x: 0, y: 0 };

    assert_eq!(origin.to_string(), "The point is (0, 0)");
    assert_eq!(format!("{}", origin), "The point is (0, 0)");

    println!("{}", origin);
    println!("Success!");
}
