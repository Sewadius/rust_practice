use core::panic;

// Unpack values from enum value
enum Message {
    _Quit,
    Move {x: i32, y: i32},
    Write(String),
    _ChangeColor(i32, i32, i32)
}
fn main() {
    let msg: Message = Message::Move { x: 1, y: 1 };

    if let Message::Move { x: a, y: b} = msg {
        assert_eq!(a, b);
    } else {
        panic!("Never let this run!");
    }

    println!("Success!");
}
