// Example for match
#[allow(dead_code)]
enum Direction {
    East,
    West,
    North,
    South
}

fn main() {
    let dire: Direction = Direction::South;

    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North => {
            println!("South or North");
        },
        _ => print!("West")
    };
}
