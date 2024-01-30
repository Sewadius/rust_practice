// Match we can use in assignments
fn main() {
    let boolean: bool = true;
    let binary: u8 = match boolean {
        true => 1,
        false => 0
    };
    assert_eq!(binary, 1);

    println!("boolean: {}", boolean);
    println!("binary: {}", binary);
}
