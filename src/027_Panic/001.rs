// Explicit panic!()
fn main() {
    drink("lemonade");
    println!("Exercise failed if printing out this line!");
}

fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Success!");
        panic!();
    }
    println!("Exercise failed if printing out this line!");
}
