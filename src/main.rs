// COMMON COLLECTIONS
// Summary :
// Vectors , strings , and hash maps will provide a large ammount of functionality necessary in
// programes when you need to stre , accesss , and modify data .
//
// Exercises :
//
// Exercise 1 :
// Server Status Code Analyzer.
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
// Exercise 3 :
// Using a hash map and Vectors , create a text interface to allow a user to add employee names to a
// department in a company. For example , "Add sally to Engineering" or " Add lotfi to Sales" . Then
// let the user retrive a list of all people in a department or all people in the company by
// department,sorted alphabetically .

use std::{collections::HashMap, io};

// Main function
fn main() {
    // HashMap storing department names as keys and a vector of employee names as values
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!(
            "\nEnter command (e.g., 'Add Lotfi to Engineering', 'View Engineering', or 'quit'):"
        );
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        if input.eq_ignore_ascii_case("quit") {
            break;
        }

        let words: Vec<&str> = input.split_whitespace().collect();
        if words.is_empty() {
            continue;
        }
        // Handling the Add name to department command
        if words.len() >= 4 && words[0].eq_ignore_ascii_case("add") {
            let name = words[1].to_string();
            let department = words[3].to_string();
            // Get or insert the vector for this department , push the name , and sort it
            let employees = company.entry(department.clone()).or_default();
            employees.push(name.clone());
            employees.sort(); // Keeps the employees list sorted alphabetically
            println!("Added {} to {}.", name, department);
        }
        // Handling the View department command :
        else if words.len() == 2 && words[0].eq_ignore_ascii_case("view") {
            let department = words[1];
            if let Some(employees) = company.get(department) {
                println!("Employees in {}: {:?}", department, employees);
            } else {
                println!("No employees found in {}", department);
            }
            // Handling the request to view all departements sorted
        } else if input.eq_ignore_ascii_case("all") {
            for (department, employees) in &company {
                println!("{}: {:?}", department, employees);
            }
        } else {
            println!("Invalid command format.");
        }
    }
}
