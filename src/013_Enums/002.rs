// Different type in enum
enum Message {
    _Quit,
    Move {x: i32, y: i32},
    Write(String),
    _ChangeColor(i32, i32, i32)
}
fn main() {
    let _msg1 = Message::Move { x: 1, y: 2 };
    let _msg2: Message = Message::Write(String::from("hello, world"));

    println!("Success!");
}
