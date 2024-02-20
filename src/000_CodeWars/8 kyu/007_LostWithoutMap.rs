use std::io;

// Lost Without a Map
fn main() {
    println!("The program returns a new array with each value doubled.\n");

    let vec: Vec<i32> = vec![1, 2, 3];
    let result: Vec<i32> = maps(&vec);

    println!("Initial vector is: {:?}", vec);
    println!("The result vector is: {:?}", result);
    wait_key_pressed();        
}

fn maps(values: &Vec<i32>) -> Vec<i32> {
    values.into_iter().map(|el| el * 2).collect()
}

fn wait_key_pressed() {
    println!("\nPress any key to continue...");
    io::stdin().read_line(&mut String::new()).unwrap();
}
