use std::io;

// A. Юный физик - 1000
fn main() {
    let mut forces: [i32; 3] = [0, 0, 0];

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let num_cases: usize = input.trim().parse().unwrap();

    for _ in 0..num_cases {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        
        let data: Vec<&str> = input.trim().split_whitespace().collect();

        for (i, force) in data.into_iter().enumerate() {
            forces[i] += force.parse::<i32>().unwrap();
        }
    }

    check_result(forces);
}

fn check_result(forces: [i32; 3]) {
    match forces {
        [0, 0, 0] => println!("YES"),
        _ => println!("NO"),
    };
}
