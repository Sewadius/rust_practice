// panic! for explicit panic
fn main() {
    never_return();

    // println!("Failed!");
}

fn never_return() -> ! {
    panic!()
}
