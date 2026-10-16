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
//
// Only Inserting a Value If the key has No value :
// It's commom to check whether a particular key has a value and , if it doesn't ,insert a value for
// it . Hash Maps have a special API for this called entry that takes the key we want to check as a
// parameter . The return value of the entry method is an enum called Entry that represents a value
// that might or might not exist .
// Let's say we want to check wether the key for the Yellow team has a value associated with it .
// If it doen't exist , we want to insert the value 50, and the same for the Blue team .
// Main function
fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    // Using the Entry method to only insert if the key does not already have a value
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);
}
