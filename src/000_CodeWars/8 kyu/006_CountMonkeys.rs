use std::io::{self, Write};

// Count the Monkeys
fn main() {
   println!("The program counts the monkeys.\n");

   print!("Enter the number: ");
   io::stdout().flush().unwrap();

   let mut s: String = String::new();
   io::stdin().read_line(&mut s).unwrap();

   let number: i32 = s.trim().parse().unwrap();
   let result: Vec<i32> = monkey_count(number);

   println!("The result is: {:?}", result);

   println!("\nPress any key to continue...");
   io::stdin().read_line(&mut String::new()).unwrap();
}

fn monkey_count(n: i32) -> Vec<i32> {
    (1..=n).into_iter().collect()
}
