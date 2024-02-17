// Lifetime for references and value
fn main() {
    let i: i32 = 3;

    {
        let borrow1: &i32 = &i;
        println!("borrow1: {}", borrow1);
    }

    {
        let borrow2: &i32 = &i;
        println!("borrow2: {}", borrow2);
    }
}
