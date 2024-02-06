use std::io::{self, Write};

trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

struct Circle { radius: f64 }

struct Rectangle { length: f64, width: f64 }

struct Triangle { side1: f64, side2: f64, side3: f64 }

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.width
    }
    fn perimeter(&self) -> f64 {
        2.0 * (self.length + self.width)
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        let p: f64 = 0.5 * (self.side1 + self.side2 + self.side3);
        (p * (p - self.side1) * (p - self.side2) * (p - self.side3)).sqrt()
    }
    fn perimeter(&self) -> f64 {
        self.side1 + self.side2 + self.side3
    }
}

// The program calculates the area and perimeter of the selected figure
fn main() {
    print_information();

    let mut chosen: String = String::new();
    io::stdin().read_line(&mut chosen).expect("Failed to read line");
    let user_choice: &str = chosen.trim();

    match user_choice {
        "0"=> circle_calculation(),
        "1" => rectangle_calculation(),
        "2" => triangle_calculation(),
        _ => {
            println!("Wrong choice! Try again");
        }
    };

    wait_for_key_press();
}

/// Calculation for circle variant
fn circle_calculation() {
    print!("\nEnter the circle's radius: ");
    let _ = io::stdout().flush();

    let parse_radius: Option<f64> = get_f64_value();
    circle_process_data(parse_radius);
}

/// Processing with final data for circle 
fn circle_process_data(parse_radius: Option<f64>) {
    if let Some(radius) = parse_radius {
        if radius > 0.0 {
            let circle: Circle = Circle { radius };
        
            println!("\nCircle's area is: {:.2}", circle.area());
            println!("Circle's perimeter is: {:.2}", circle.perimeter());
        } else {
            println!("Error! You've entered a negative radius!");
            wait_for_key_press();
        }
    } else {
        wait_for_key_press();
    }
}

/// Processing for rectangle variant
fn rectangle_calculation() {
    print!("\nEnter the rectangle's length: ");
    let _ = io::stdout().flush();

    let parse_length: Option<f64> = get_f64_value();
    if let Some(length) = parse_length {
        if length > 0.0 {
            print!("Enter the rectangle's width: ");
            let _ = io::stdout().flush();

            let parse_width: Option<f64> = get_f64_value();

            if let Some(width) = parse_width {
                if width > 0.0 {
                    rectangle_process_data(length, width);
                } else {
                    println!("Error! The rectangle's width must be positive!");
                }
            }
        } else {
            println!("Error! The rectangle's length must be positive!");
        }
    } 
}

/// Processing with final data for rectangle 
fn rectangle_process_data(length: f64, width: f64) {
    let rectangle: Rectangle = Rectangle { length, width };

    println!("\nRectangle's area is: {:.2}", rectangle.area());
    println!("Rectangle's perimeter is: {:.2}", rectangle.perimeter());
}

/// Processing for triangle variant
fn triangle_calculation() {
    print!("\nEnter the sides of the triangle on the same line, separated by a space: ");
    let _ = io::stdout().flush();

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let data: Vec<&str> = input.split(' ').collect();
    match data.len() == 3 {
        true => triangle_process_data(data),
        false => {
            println!("Error! Wrong number of sides! {}, but expected 3", data.len());
        }
    }
}

/// Processing with data for triangle
fn triangle_process_data(data: Vec<&str>) {
    let mut values: Vec<f64> = Vec::new();

    for side in data {
        let value: Option<f64> = match side.trim().parse::<f64>() {
            Ok(input) => Some(input),
            Err(_) => {
                println!("Error! Failed to parse to float number!");
                None
            }
        };

        if let Some(triangle_side) = value {
            values.push(triangle_side);
        } else { return; }
    }

    match check_is_a_triangle(values.clone()) {
        true => triangle_get_result(values),
        false => println!("Error! This is not a valid triangle!")
    };
}

/// Checking for valid triangle by its sides
fn check_is_a_triangle(mut values: Vec<f64>) -> bool {
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    values[2] < values[0] + values[1]
}

/// Final result for triangle variant
fn triangle_get_result(values: Vec<f64>) {
    let (side1, side2, side3) = (values[0], values[1], values[2]);
    let triangle: Triangle = Triangle { side1, side2, side3 };

    println!("\nTriangle's area: {:.2}", triangle.area());
    println!("Triangle's perimeter: {:.2}", triangle.perimeter());
}

/// Converts String input to Option<f64> with parse
fn get_f64_value() -> Option<f64> {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim().parse::<f64>() {
        Ok(input) => Some(input),
        Err(_) => {
            println!("Error! Failed to parse to float number!");
            None
        }
    }
}

/// Prints welcome and info for user
fn print_information() {
    const FIGURES: [&str; 3] = [
        "Circle", "Rectangle", "Triangle"
    ];

    println!("The program calculates the area and perimeter of the selected figure.\n");

    print!("Please choose the number from this list [0 - {}, 1 - {}, 2 - {}]: ", 
        FIGURES[0], FIGURES[1], FIGURES[2]);
    
    let _ = io::stdout().flush();
}

/// Waiting for key pressed from user
fn wait_for_key_press() {
    println!("\nPress any key to continue...");
    io::stdin().read_line(&mut String::new()).expect("Failed to read");
}
