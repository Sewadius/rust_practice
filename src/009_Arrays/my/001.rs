// The program creates an array of length 10 elements i32 with a given step size
use std::io;
use std::io::Write;
use std::io::stdin;

#[macro_use]
mod macros {
    #[macro_use]
    mod custom_io;
}

const LENGTH: usize = 10;

fn main() {
    println!("The program creates an array of length 10 elements i32 with a given step size.\n");

    print!("Enter the step: ");
    flush_stdout!();

    let step: i32 = get_i32_value!();
    let arr: [i32; LENGTH] = create_array(step);

    println!("Result array: {:?}", arr);
    wait_for_key_press!();
}

fn create_array(step: i32) -> [i32; LENGTH] {
    if step == 0 {
        return [0; LENGTH];
    }
    
    let mut arr: [i32; LENGTH] = [0; LENGTH];
    for i in 0..LENGTH {
        arr[i] = (i as i32 + 1) * step;
    }

    arr
}
