use std::collections::HashMap;

// COMMON COLLECTIONS
// Updating a Hash Map :
// Although the number of keys and values is growable , each key can only have one value associated
// with it at a time . When you want to change the data in a hash map, you have to decide how to
// handle the case when a key has already a value assigned . We could replae the old value with the
// new value , completely disregarding the old value . We could keep the old value and ignore the
// new value . Or we could combine the old value and the new value.
// Overwriting a value :
// If we insert a key and a value into a hash map and then insert that same key with different value
// , the value associated with that key will be replaced .
// Even though the code in the last example calls insert twice , the hash map will only contain one
// key/value pair because we're inserting the value for the Blue team's key both times .
// Main function
fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    // Replacing a value stored with a particular key
    scores.insert(String::from("Blue"), 24);
    println!("{:?}", scores);
}
