// Using for struct array
#[allow(dead_code)]
struct Array<T, const N: usize> {
    data: [T; N]
}


fn main() {
    let arrays: [Array<i32, 3>; 3] = [
        Array { data: [1, 2, 3] },
        Array { data: [1, 2, 3] },
        Array { data: [4, 5, 6]}
    ];

    let floats: [Array<f64, 2>; 3] = [
        Array { data: [-1.0, 2.0] },
        Array { data: [1.12, 4.32] },
        Array { data: [0.2, -9.12] }
    ];

    println!("arrays length: {}", arrays.len());
    println!("floats length: {}", floats.len());
}
