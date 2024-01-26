// Statement & expression
fn main() {
    let x: u32 = 5u32;

    let y = {
        let x_squared = x * x;          // 25
        let x_cube = x_squared * x;     // 125

        // Expression assigned to `y`
        x_cube + x_squared + x          // 125 + 25 + 5 = 155
    };

    let z = {
        let _ = 2 * x;
    };

    println!("x is {:?}", x);   // 5
    println!("y is {:?}", y);   // 155
    println!("z is {:?}", z);   // ()
}
