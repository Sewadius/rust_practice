// Size of slices
fn main() {
    let arr: [char; 3] = ['a', 'b', 'c'];
    let slice: &[char] = &arr[..2];                 // First two elements

    println!("{:?}", slice);
    println!("Size of slice: {}", std::mem::size_of_val(&slice));
    assert!(std::mem::size_of_val(&slice) == 16);   // 2 bytes for char and length
    println!("Success!");
}
