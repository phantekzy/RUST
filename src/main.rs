use std::{collections::HashMap, vec};

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
    // Another way of constructing a hash map is by using the collect method on a vector of tuples ,
    // where each tuple consists of a key and its value . the collect method gathers data into a
    // number of colection types , including HashMap . For example : if we had the team names and
    // initial scores in two separate vectors , we could use the zip method to create a vector of
    // tuples where "blue" is paired with 10 , and so forth . Then we can use the collect method to
    // turn that vector of tuples into a hash map .
    let teams = vec![String::from("blue"), String::from("Green")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
}
