// Array with trait objects
trait Bird {
    fn quack(&self);
}

struct Duck;
#[allow(dead_code)]
impl Duck {
    fn fly(&self) {
        println!("Look, the duck is flying");
    }
}

struct Swan;
#[allow(dead_code)]
impl Swan {
    fn fly(&self) {
        println!("Look, the duck...oh sorry, the swan is flying");
    }
}

impl Bird for Duck {
    fn quack(&self) {
        println!("{}", "duck duck");
    }
}

impl Bird for Swan {
    fn quack(&self) {
        println!("{}", "swan swan");
    }
}
fn main() {
    let birds: [&dyn Bird; 2] = [&Duck, &Swan];

    for bird in birds {
        bird.quack();
    }
}
