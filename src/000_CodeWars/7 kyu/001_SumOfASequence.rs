// Sum of sequence
fn main() {
    println!("A function returns the sum of a sequence of integers.\n");

    assert_eq!(2, sequence_sum(2, 2, 2));
    assert_eq!(15, sequence_sum(1, 5, 1));
    assert_eq!(5, sequence_sum(1, 5, 3));

    println!("Success!");
}

fn sequence_sum(start: u32, end: u32, step: u32) -> u32 {
    (start..=end).step_by(step as usize).sum()
}
