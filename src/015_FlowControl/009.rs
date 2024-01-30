// Infinite loop
fn main() {
    let mut count: u32 = 0u32;

    loop {
        count += 1;

        if count == 3 {
            println!("Three");
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("Ok, that's enough!");
            break;
        }
    }

    assert_eq!(count, 5);
    println!("Success!");
}
