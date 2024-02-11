use std::collections::HashMap;

// HashMap capacity
fn main() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    
    assert!(map.capacity() >= 100);

    map.shrink_to(50);
    assert!(map.capacity() >= 50);

    map.shrink_to_fit();
    assert!(map.capacity() >= 2);

    println!("Success!");
}
