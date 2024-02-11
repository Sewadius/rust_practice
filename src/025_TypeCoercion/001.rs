// Casting from float to char
fn main() {
    let decimal: f32 = 97.123_f32;
    let integer: u8 = decimal as u8;       // 97

    let c1: char = decimal as u8 as char;  // 'a'
    let c2: char = integer as char;

    println!("c1 = {}, c2 = {}", c1, c2);

    assert_eq!(integer + 1, 'b' as u8);
    println!("Success!"); 
}
