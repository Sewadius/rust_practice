// Different impl blocks
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {width: 100, height: 100};
    let rect2 = Rectangle {width: 90, height: 90};

    println!("Rectangle 1 area: {}", rect1.area());
    println!("Rectangle 1 can hold rectangle 2: {}", rect1.can_hold(&rect2));
}
