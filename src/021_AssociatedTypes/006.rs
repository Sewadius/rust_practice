// Object save into traits, dynamic dispatch
trait MyTrait {
    fn f(&self) -> Box<dyn MyTrait>;
}

impl MyTrait for u32 {
    fn f(&self) -> Box<dyn MyTrait> { Box::new(42) }
}

impl MyTrait for String {
    fn f(&self) -> Box<dyn MyTrait> { Box::new(self.clone()) }
}

fn main() {
    my_function(Box::new(13_u32));
    my_function(Box::new(String::from("hello")));

    println!("Success!");
}

fn my_function(x: Box<dyn MyTrait>) -> Box<dyn MyTrait> { 
    x.f() 
}
