// Different types: () and i32
fn main() {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };

    let v2: () = {
        let mut _x: i32 = 1;
        _x += 1;
    };

    assert_eq!(v, 3);
    assert_eq!(v2, ());

    println!("Success!");
}
