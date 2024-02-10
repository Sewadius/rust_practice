use std::collections::HashMap;

// HashMap main methods
fn main() {
    let mut scores = HashMap::new();
    scores.insert("Sunface", 98);
    scores.insert("Daniel", 95);
    scores.insert("Ashley", 69);
    scores.insert("Katie", 58);

    let score: Option<&i32> = scores.get("Sunface");
    assert_eq!(score, Some(98).as_ref());

    if scores.contains_key("Daniel") {
        let score: i32 = scores["Daniel"];
        assert_eq!(score, 95);
        scores.remove("Daniel");
    }

    assert_eq!(scores.len(), 3);

    for (name, score) in scores {
        println!("The score of {} is {}", name, score);
    }
}
