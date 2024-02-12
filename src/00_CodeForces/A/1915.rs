use std::{collections::HashSet, io};

// A. Выброс - 800
fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Error");
    let counter: usize = input.trim().parse::<usize>().unwrap();

    for _ in 0..counter {
        let mut line: String = String::new();
        io::stdin().read_line(&mut line).expect("Error");

        let numbers: Vec<&str> = line.trim().split(' ').collect();
        let mut mask: HashSet<usize> = HashSet::new();

        for elem in numbers {
            let number: usize = elem.parse().unwrap();

            if mask.contains(&number) {
                mask.remove(&number);
            } else {
                mask.insert(number);
            }
        }

        println!("{}", mask.iter().next().unwrap());
    }
}
