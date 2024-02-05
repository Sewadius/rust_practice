// The program creates and processes a rectangle structure
trait Figure {
    fn new(width: u32, height: u32) -> Self;
    fn area(&self) -> u32;
    fn is_square(&self) -> bool;
}
struct Rectangle {
    width: u32,
    height: u32
}
impl Figure for Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Rectangle {width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}
fn main() {
    println!("The program creates and processes a rectangle structure.\n");

    let rect_1: Rectangle = Rectangle::new(100, 120);
    let rect_2: Rectangle = Rectangle::new(50, 50);

    assert_eq!(12000, rect_1.area());
    assert_eq!(false, rect_1.is_square());

    assert_eq!(2500, rect_2.area());
    assert_eq!(true, rect_2.is_square());

    println!("Success!");
}
