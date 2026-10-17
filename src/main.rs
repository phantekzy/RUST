use std::collections::HashMap;

// COMMON COLLECTIONS
// Updating a Value Based on the Old Value :
// Another common use case for hash maps is to look up key's value and then update it based on the
// old value . For instance this code it counts how many times each word appears in some text .
// We use a hash map with the words as keys and increment the value to keep track of how many times
// we've seen that word . If it's the first time we've seen a word , we'll insert the value 0.
// Main function
fn main() {
    let text = "Hello world wonderful world";
    let mut map = HashMap::new();
    // Counting occurences of words using a hash map that stores words and counts
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
    // The code will print {"Hello": 1, "wonderful": 1, "world": 2} . The or_insert method actually
    // returns a mutable reference (&mut v) to the value for this key .
}
