#[derive(Debug)]
struct Structure(i32);

// Using Debug trait for printing
fn main() {
    println!("{} months in a year.", 12);
    println!("Now {:?} will print!", Structure(3));
}
