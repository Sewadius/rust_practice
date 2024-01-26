// Computations
fn main() {
    // Integers
    assert!(1u32 + 2 as u32 == 3);
    assert!(1i32 - 2 as i32 == -1);
    assert!(1i8 - 2 as i8 == -1);
    assert!(3 * 50 == 150);             // i32

    // Float
    assert!(9.6 as f32 / 3.2 as f32 == 3.0);

    assert!(24 % 5 == 4);

    // Boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);

    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    println!("Success!");
}
