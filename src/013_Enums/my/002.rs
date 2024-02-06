use std::io;

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West
}

// The program changes the direction of the sides of the world
fn main() {
    println!("The program changes the direction of the sides of the world.\n");

    let mut current_direction: Direction = Direction::North;
    println!("Current direction: {:?}\n", current_direction);

    for _ in 0..4 {
        let direction: Direction = turn(&current_direction);
        println!("Next direction after turning clockwise: {:?}", direction);
        current_direction = direction;
    }
    
    println!("\nPress any key to continue...");
    io::stdin().read_line(&mut String::new()).expect("Failed");
}

/// Change direction after clockwise turn
fn turn(current: &Direction) -> Direction {
    match current {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North
    }
}
