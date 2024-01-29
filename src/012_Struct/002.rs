// Tuple structure
struct Color(i32, i32, i32);
struct _Point(i32, i32, i32);

fn main() {
    let v: Color = Color(0, 127, 255);
    check_color(v);

    println!("Success!");
}

fn check_color(p: Color) {
    let Color(x, _, z) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(z, 255);
}
