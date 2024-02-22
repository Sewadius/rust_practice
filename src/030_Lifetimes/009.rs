// Using lifetime in user's structs & enums
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

#[derive(Debug)]
#[allow(dead_code)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}
fn main() {
    let x: i32 = 18;
    let y: i32 = 15;

    let single: Borrowed = Borrowed(&x);
    let double: NamedBorrowed = NamedBorrowed { x: &x, y: &y };
    let reference: Either = Either::Ref(&x);
    let number: Either = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}
