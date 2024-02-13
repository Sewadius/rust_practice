// Result example
fn main() {
    let result_1: Result<f32, &str> = divide(10.0, 2.0);
    let result_2: Result<f32, &str> = divide(15.0, 0.0);

    for el in [result_1, result_2] {
        handle_divide_result(el);
    }
}

fn divide(x: f32, y: f32) -> Result<f32, &'static str> {
    if y == 0.0 {
        return Err("Division by zero!");
    }
    Ok(x / y)
}

fn handle_divide_result(result: Result<f32, &'static str>) {
    match result {
        Ok(val) => println!("Result: {}", val),
        Err(msg) => println!("Error: {}", msg),
    };
}
