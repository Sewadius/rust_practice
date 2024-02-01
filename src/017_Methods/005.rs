// Using method with enums
#[allow(dead_code)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green
}

impl TrafficLightColor {
    pub fn color(&self) -> &str {
        match self {
            Self::Red => "red",
            Self::Yellow => "yellow",
            Self::Green => "green"
        }
    }
}
fn main() {
    let c: TrafficLightColor = TrafficLightColor::Yellow;
    assert_eq!(c.color(), "yellow");

    println!("Success!");
    println!("{:?}", c.color());
}
