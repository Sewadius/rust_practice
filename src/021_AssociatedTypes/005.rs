// Object save into traits, static dispatch
trait MyTrait {
    fn f(&self) -> Self;
}

impl MyTrait for u32 {
    fn f(&self) -> Self { 42 }
}

impl MyTrait for String {
    fn f(&self) -> Self { self.clone() }
}

fn main() {
    println!("{}", my_function(13_u32));
    println!("{}", my_function(String::from("hello")));

    println!("Success!");
}

fn my_function<T: MyTrait>(x: T) -> T { 
    x.f() 
}
