use std::collections::HashMap;

// Operations with entries in HashMap
fn main() {
    let mut player_stats: HashMap<&str, i32> = HashMap::new();

    player_stats.entry("Health").or_insert(100);
    assert_eq!(player_stats["Health"], 100);

    player_stats.entry("Health").or_insert_with(random_stat_buff);
    assert_eq!(player_stats["Health"], 100);

    let health: &mut i32 = player_stats.entry("Health").or_insert(50);
    assert_eq!(health, &100);

    *health -= 50;
    assert_eq!(*health, 50);

    println!("Success!");
}

fn random_stat_buff() -> i32 {
    42
}
