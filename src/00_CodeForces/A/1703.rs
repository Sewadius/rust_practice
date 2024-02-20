use std::io;

// A. YES или YES? - 800
fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: usize = input.trim().parse().unwrap();

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_lowercase();

        match input.as_str() {
            "yes" => println!("YES"),
            _ => println!("NO")
        };
    }
}
