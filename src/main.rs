// COMMON COLLECTIONS
// Exercises :
// Exercise 3 :
// Using a hash map and Vectors , create a text interface to allow a user to add employee names to a
// department in a company. For example , "Add sally to Engineering" or " Add lotfi to Sales" . Then
// let the user retrive a list of all people in a department or all people in the company by
// department,sorted alphabetically .

use std::{collections::HashMap, io};

// Main function
fn main() {
    // Creating a HashMap to store department names as keys and vector to store employees as values
    let mut _company: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!(
            "Enter command (Example : 'Add Lotfi to Engineering' , 'View Engineering' , or 'Quit or exit ')"
        );
        // The input variable and it will be a String and we should make it mutable
        let mut input = String::new();
        // We should read the line command and not to forget the case where we don't have any
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line");
        let input = input.trim(); // Shadowing the first input variable and trim it 
        // the first case : When the user decides to type Quit and exit the program
        if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
            break;
        }
        // Collecting the input into a slice of string
        let Words: Vec<&str> = input.split_ascii_whitespace().collect();
        if Words.is_empty() {
            continue; // continue is only used in loops and it goes back to the top of the loop 
        }
        // The add case
        if Words.len() >= 4 && Words[0].eq_ignore_ascii_case("add") {
            let name = Words[1].to_string();
            let department = Words[3].to_string();
            // Get or insert the vector for this department, push the name, and sort it
        }
    }
}
