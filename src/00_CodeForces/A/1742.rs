use std::io;

// A. Сумма - 800
fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let cases: usize = input.trim().parse().unwrap();

    for _ in 0..cases {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let mut vec: Vec<usize> = input.trim().split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        vec.sort();

        match vec[2] == vec[0] + vec[1] {
            true => println!("YES"),
            false => println!("NO"),
        }
    };
}
