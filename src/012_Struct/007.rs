// Lost owner for structure's field "name"
#[derive(Debug)]
struct File {
    name: String,
    data: String
}

fn main() {
    let f: File = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string()
    };

    let name: String = f.name;
    println!("{}, {}", name, f.data);
}
