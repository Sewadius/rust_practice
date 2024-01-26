// Two ways not to take the ownership
fn main() {
    let s: String = String::from("hello, world");
    print_str(&s);
    print_str_2(s.clone());

    println!("{}", s);
}

fn print_str(s: &String) {
    println!("{}", s)
}

fn print_str_2(s: String) {
    println!("{}", s)
}
