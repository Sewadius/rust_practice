// Usinng value and referene in one scope
fn main() {
    {
        let x: i32 = 5;
        let r: &i32 = &x;

        println!("r: {}", r);
    }
}
