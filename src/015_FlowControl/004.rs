// Iteration via array's elements
fn main() {
    let names: [String; 2] = [String::from("liming"), String::from("hanmeimei")];
    for name in &names {
        println!("{}", name);
    }

    let numbers: [i32; 3] = [1, 2, 3];
    for n in numbers {
        print!("{} ", n);
    }

    println!("{:?}", names);
}
