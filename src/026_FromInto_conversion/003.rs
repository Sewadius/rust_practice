// Conversing between types
fn main() {
    let i1: i32 = false.into();
    let i2: i32 = i32::from(false);

    assert_eq!(i1, i2);
    assert_eq!(i1, 0);

    let i3: u32 = 'a'.into();
    println!("'a' = {}", i3);

    let s1: String = String::from('a');
    let s2: String = 'a'.into();

    println!("s1 = {}, s2 = {}", s1, s2);

    assert_eq!(&s1, "a");
    println!("Success!");
}
