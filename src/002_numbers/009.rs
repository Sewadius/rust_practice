// Char output as number codes
fn main() {
    let mut sum: i32 = 0;

    for i in -3..2 {
        sum += i
    }

    println!("sum = {}", sum);
    assert!(sum == -5);

    for c in 'a'..='z' {            // 'z' included
        print!("{} ", c as u8);     // Number codes  
    }
}
