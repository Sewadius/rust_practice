// Shadowing variable and reuse it
fn main() {
    let age: Option<i32> = Some(30);

    if let Some(age) = age {
        assert_eq!(age, 30);
    }

    match age {
        Some(age) => println!("Age is a new variable, it's value is {}", age),
        _ => ()
    }
}
