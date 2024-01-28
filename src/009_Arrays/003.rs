// Short syntax to initialize all elements
fn main() {
    let list: [i32; 100] = [1; 100];

    assert!(list[0] == 1);
    assert!(list[99] == 1);
    assert!(list.len() == 100);

    println!("Success!");
}
