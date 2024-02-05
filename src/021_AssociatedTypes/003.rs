// &dyn and Box<dyn>
trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", self)
    }
}
fn main() {
    let x: f64 = 1.1f64;
    let y: u8 = 8u8;

    println!("{}", x.draw());
    println!("{}", y.draw());

    println!();

    println!("{}", draw_with_box(Box::new(x)));
    println!("{}", draw_with_ref(&y));
}

fn draw_with_box(x: Box<dyn Draw>) -> String {
    x.draw()
}

fn draw_with_ref(x: &dyn Draw) -> String {
    x.draw()
}
