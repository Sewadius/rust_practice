// Using lifetime in functions 
fn main() {
    let x: i32 = 7;
    let y: i32 = 9;

    print_one(&x);
    print_multi(&x, &y);

    let z: &i32 = pass_x(&x, &y);
    print_one(z);

    let mut t: i32 = 3;
    add_one(&mut t);
    print_one(&t); 
}

fn print_one<'a>(x: &'a i32) {
    println!("`print_one`: x is {}", x);
}

fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("`print_multi`: x is {}, y is {}", x, y);
}

fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 { x }
