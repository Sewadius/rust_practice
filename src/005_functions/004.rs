// Differents ways to panic
fn main() {
    get_option(2);
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            println!("First");
        }
        _ => {
            println!("Another");
        }
        
    };

    never_return_fn();
}

fn never_return_fn() -> ! {
    // panic!()
    todo!()
    // unimplemented!()
}
