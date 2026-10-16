use std::collections::HashMap;

// COMMON COLLECTIONS
// Storing Keys with Associated Values in Hash maps
// The last of our common collections is the hash map. The Type HashMap<k , v> stores a mapping of
// keys of type k to values of type V.
// It does this via a hashing function , which determines how it places these keys and values into
// memory.
// Many programming languages support this kind of data structure , but they often use a different
// name , as hash , map , object , hash table , dictionary , or assoicative array.
// Hash maps are usefull when you want to look up data not by using an index , as you can with
// vectors , but by using a key that can be of any type.
// For example , in a game , we could keep track of each team's score in a hash map in which each
// key is a team's name and the values are each team's score ,
// Given a team name , we can retrieve its score .
// We'll go over the basic API of hash maps in this section , but many more goodies are hiding in
// the functions defined on HashMap<K,V> by the standard library.
//
// Main function
fn main() {
    // Creating a New Hash map
    // We can create an empty hash map with new and add element with insert .
    // Creating a new Hash map
    let mut scores = HashMap::new();
    // Inserting some keys and values
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Green"), 50);
    // Note that we need to first use the hachmap from the collections portion of the standard
    // library . Of our three  common collections ,this is the least often used , so it's not
    // inclueded in the features brought into scope automatically in the prelude .
}
