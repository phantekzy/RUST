use std::collections::HashMap;

// COMMON COLLECTIONS
// Summary :
// Vectors , strings , and hash maps will provide a large ammount of functionality necessary in
// programes when you need to stre , accesss , and modify data .
// Exercises :
// Exercise 1 : Server Status Code Analyzer.
// Imagine you are analyzing a list of HTTP status codes collected from a web server log over a brief period.
// You have a vector of integers:
// let codes = vec![200, 404, 200, 500, 404, 200, 403, 404, 200, 500, 500];
// Write a Rust program that processes this vector and performs three tasks:
//  Find the Extremes: Find the minimum and maximum status codes in the vector. (Hint: You can sort the vector or iterate through it tracking min and max values).
//  Build a Frequency Map: Create a HashMap using the .entry() and .or_insert() API to count how many times each status code appears in the log.
//  Find the Most Common Code: Iterate through your hash map to find which status code occurred most frequently (the mode).
//
//
// Main function
fn main() {
    let mut sales = HashMap::new();
    let flavors = vec!["Chocolate", "Vanilla", "Chocolate", "Strawberry", "Vanilla"];
    for flavor in flavors {
        let count = sales.entry(flavor).or_insert(0);
        *count += 1;
    }
    println!("{:?}", sales);
}
