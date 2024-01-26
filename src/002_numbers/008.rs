// Float sum assert
fn main() {
    assert!(0.1_f32 + 0.2_f32 == 0.3);          // Fist way
    assert!(0.1 as f32 + 0.2 as f32 == 0.3);    // Second way
    println!("Success!");
}
