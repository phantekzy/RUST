use std::collections::HashMap;

// COMMON COLLECTIONS
// Accessing value in a HashMap :
// Main function
fn main() {
    // We can get a value out of the hash map by providing its key to the get method :
    let mut scores = HashMap::new();
    scores.insert(String::from("FC Barcelona"), 4);
    scores.insert(String::from("Real Madrid CF"), 2);

    let team_name = String::from("FC Barcelona");
    // Accessing the score for the Fc Barcelona team stored on the HashMap
    let score = scores.get(&team_name);
    println!("{:?}", score);
    // Here , score will have the value that's associated with the blue team, and the result will be
    // Some(4); The result is wrapped in Some because get returns Option<&V>; if there's no value
    // for that key in the map , get will return None .
    // We can iterat eover each key/value pair in a hash map :
}
