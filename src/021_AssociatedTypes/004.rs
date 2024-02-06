// Static and dynamic dispatch
trait Foo {
    fn method(&self) -> String;
}
impl Foo for u8 {
    fn method(&self) -> String {
        format!("u8: {}", *self)
    }
}
impl Foo for String {
    fn method(&self) -> String {
        format!("string: {}", *self)
    }
}
fn main() {
    let x: u8 = 5u8;
    let y: String = "Hello".to_string();

    println!("{}", static_dispatch(x));
    println!("{}", dymamic_dispatch(&y));

    println!("Success!");
}

fn static_dispatch<T: Foo>(a: T) -> String {
    a.method()
}

fn dymamic_dispatch(a: &dyn Foo) -> String {
    a.method()
}
