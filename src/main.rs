use std::collections::HashMap;

// COMMON COLLECTIONS
// Updating a Value Based on the Old Value :
// Another common use case for hash maps is to look up key's value and then update it based on the
// old value . For instance this code it counts how many times each word appears in some text .
// We use a hash map with the words as keys and increment the value to keep track of how many times
// we've seen that word . If it's the first time we've seen a word , we'll insert the value 0.
// Main function
fn main() {
    // Creating a slice string
    let message = "You are stupid as hell and hell is not hotter than u , stupid as hell and hell is not stupid as man like u stupid man ";
    // Creating a mutable empty HashMap
    let mut map = HashMap::new();

    // Looping on the words of the message :
    for word in message.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
}
