// Debug output for Struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let scale: u32 = 2;
    let rect1: Rectangle = Rectangle {
        width: dbg!(30 * scale),
        height: 50
    };

    dbg!(&rect1);
    println!("{:?}", rect1);
}
