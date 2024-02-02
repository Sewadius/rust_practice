// Using derive for debug, check equal and compare
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches)  = self;
        Centimeters(inches as f64 * 2.54)
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Seconds(i32);

fn main() {
    let _one_second: Seconds = Seconds(1);

    println!("One second looks like: {:?}", _one_second);

    let _this_is_true: bool = _one_second == _one_second;
    println!("_one_second == _one_second: {}", _this_is_true);

    let _this_is_false: bool = _one_second > _one_second;
    println!("_one_second > _one_second: {}", _this_is_false);

    let foot: Inches = Inches(12);
    println!("\nOne foot equals {:?}", foot);

    let meter: Centimeters = Centimeters(100.0);

    let cmp: &str = match foot.to_centimeters() < meter {
        true => "smaller",
        false => "bigger"
    };

    println!("One foot is {} than one meter.", cmp);
}
