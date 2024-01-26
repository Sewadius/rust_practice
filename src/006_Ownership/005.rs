// clone() & copy
fn main() {
    // clone() for String
    let x = (1, 2, (), "hello".to_string());
    let y = x.clone();
    println!("{:?}, {:?}", x, y);

    // Simple copy with &str
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}
