// f64 type for concrete implementation generic struct
struct Point<T> {
    x: T,
    y: T
}

impl Point<f64> {
    pub fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p: Point<f64> = Point { x: 5f64, y: 10f64 };
    println!("Distance: {}", p.distance_from_origin());
}
