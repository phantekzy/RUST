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
// Exercise 2 :
// Convert Strings to pig latin . The first consonant of each word is moved to the end of the word
// and "ay" is added , so fist becomes "irst-fay".
// Words that starts with vowel have "hay" added to the end instead ("apple" becomes "apple-hay") .
// Keep in mind the detail about UTF-8 encoding .
//
fn to_pig_latin(word: &str) -> String {
    let mut chars = word.chars();

    // Safely extract the first character using the iterator
    let first_char = match chars.next() {
        Some(c) => c,
        None => return String::new(), // Handle empty strings safely
    };

    // Define vowels for comparison
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

    if vowels.contains(&first_char) {
        // Rule for vowels: append "-hay"
        format!("{}-hay", word)
    } else {
        // Rule for consonants: collect the remaining characters,
        // move the first character to the end, and append "-ay"
        let remainder: String = chars.collect();
        format!("{}-{}ay", remainder, first_char)
    }
}

// Main function
fn main() {
    let word1 = "apple";
    let word2 = "first";

    println!("{} -> {}", word1, to_pig_latin(word1));
    println!("{} -> {}", word2, to_pig_latin(word2));
}
