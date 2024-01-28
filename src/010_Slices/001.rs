// Using & for slices
fn main() {
    let arr: [i32; 3] = [1, 2, 3];

    let _s1: &[i32] = &arr[0..2];
    let _s2: &str = "hello, world";
}
