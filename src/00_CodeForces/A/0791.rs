use std::io;

// A. Мишка и старший брат - 800
fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let weights: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    if let [mut weight_1, mut weight_2] = weights.as_slice() {
        let mut years: usize = 0;
        
        loop {
            if weight_1 > weight_2 { break; }
            weight_1 *= 3; weight_2 *= 2;
            years +=1;
        }

        println!("{}", years);
    }
}   
