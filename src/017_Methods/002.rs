// Change state of the object
struct TrafficLight {
    color: String
}

impl TrafficLight {
    pub fn show_state(&self) {
        println!("The current state is {}", self.color);
    }

    pub fn change_state(&mut self) {
        self.color = "green".to_string();
    }
}
fn main() {
    let mut traffic_light: TrafficLight = TrafficLight {
        color: String::from("red")
    };

    traffic_light.show_state();
    traffic_light.change_state();
    traffic_light.show_state();
}
