// Simple syntax for copy from another structure's object
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {
    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1
    };

    let _u2: User = set_email(u1);
    println!("Success!");
}

fn set_email(u: User) -> User {
    User {
        email: String::from("contact@in.dev"),
        ..u
    }
}
