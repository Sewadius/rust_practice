// clear() onlt after reference ended
fn main() {
    let mut s: String = String::from("hello, world");
    let word = first_letter(&s);
    println!("The first letter is {}", word);
    s.clear();
    println!("The capacity is: {}", s.capacity());
}   

fn first_letter(s: &str) -> &str {
    &s[..1]
}
