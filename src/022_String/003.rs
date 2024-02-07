// Indexing for String
fn main() {
    let s: String = String::from("hello, Nick");
    let slice1: &str = &s[..1];
    assert_eq!(slice1, "h");

    let slice2: &str = &s[5..6];
    assert_eq!(slice2, ",");

    for (i, c) in s.chars().enumerate() {
        if i == 7 {
            assert_eq!(c, 'N');
        }
    }

    println!("Success!");
}
