// Out of bounds and get()
fn main() {
    let names: [String; 2] = [String::from("Sunfei"), "Sunface".to_string()];

    let _name0: &String = names.get(0).unwrap();
    //let _name1 = &names[2];     // Out of bounds -> panic!

    println!("Success!");
}
