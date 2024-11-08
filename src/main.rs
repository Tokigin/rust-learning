use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    // Insert key-value pairs
    scores.insert("Alice", 50);
    scores.insert("Bob", 30);
    scores.insert("Charlie", 40);

    // Access a value
    if let Some(&score) = scores.get("Alice") {
        println!("Alice's score before: {}", score);
    }
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }
    // Update a value
    scores.insert("Alice", 60); // Update Alice's score

    // Remove an entry
    scores.remove("Bob");

    // Iterate over the HashMap
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }
}
