// Complex struct with lifetime using
#[derive(Debug)]
struct NoCopyType {}

#[derive(Debug)]
#[allow(dead_code)]
struct Example<'a> {
    a: &'a u32,
    b: &'a NoCopyType,
}

fn main() {
    let var_a: u32 = 35;
    let var_b: NoCopyType = NoCopyType {};

    let example: Example = Example {a: &var_a, b: &var_b };

    println!("Success! {:?}", example);
}
