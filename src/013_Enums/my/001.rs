use std::io::stdin;

#[macro_use]
mod macros {
    #[macro_use]
    mod custom_io;
}

#[derive(Debug, Clone)]
enum TrafficLight {
    Red,
    Yellow,
    Green
}

// The program uses enum, example of working with "TrafficLights"

fn main() {
    println!("The program uses enum, example of working with \"TrafficLights\".\n");

    let lights: [TrafficLight; 3] = [
        TrafficLight::Red,
        TrafficLight::Yellow,
        TrafficLight::Green
    ];

    for traffic_light in lights.to_vec() {
        let current_light:TrafficLight = traffic_light;
        let next_light: TrafficLight = next_light(current_light.clone());

        println!("Next light for {:?} is {:?}", current_light, next_light);
    }

    wait_for_key_press!();
}

fn next_light(current_light: TrafficLight) -> TrafficLight {
    match current_light {
        TrafficLight::Green => TrafficLight::Yellow,
        TrafficLight::Yellow => TrafficLight::Red,
        TrafficLight::Red => TrafficLight::Green
    }
}
