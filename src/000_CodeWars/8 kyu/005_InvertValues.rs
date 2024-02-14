use std::io;

// Returns inverse array for a given set of numbers
fn main() {
    println!("The program returns inverse array for a given set of numbers.\n");

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let vec: Vec<i32> = invert(&arr);

    println!("Initial array: {:?}", arr);
    println!("The result: {:?}", vec);

    println!("\nPress any key to continue...");
    io::stdin().read_line(&mut String::new()).unwrap();
}

fn invert(values: &[i32]) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    for &value in values {
        result.push(-value);
    }

    result
}
