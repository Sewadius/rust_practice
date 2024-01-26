// No value return, but panic
fn main() {
    let b: bool = false;

    let _v = match b {
        true => 1,
        false => {
            println!("Success!");
            panic!("we have no value for 'false', but we can panic");
        }
    };

    println!("Failed!"); 
}
