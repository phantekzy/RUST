// COMMON COLLECTIONS
// Exercises :
// Exercise 3 :
// Using a hash map and Vectors , create a text interface to allow a user to add employee names to a
// department in a company. For example , "Add sally to Engineering" or " Add lotfi to Sales" . Then
// let the user retrive a list of all people in a department or all people in the company by
// department,sorted alphabetically .

use std::{collections::HashMap, io};

fn main() {
    //  HashMap storing department names as keys and a vector of employee names as values
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!(
            "\nEnter command (e.g., 'Add Sally to Engineering', 'View Engineering', or 'quit'):"
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

        //  Handling the "Add [name] to [department]" command
        if words.len() >= 4 && words[0].eq_ignore_ascii_case("add") {
            let name = words[1].to_string();
            let department = words[3].to_string();

            // Get or insert the vector for this department, push the name, and sort it
            let employees = company.entry(department.clone()).or_default();
            employees.push(name.clone());
            employees.sort(); // Keeps the employee list sorted alphabetically

            println!("Added {} to {}.", name, department);

        //  Handling the "View [department]" command
        } else if words.len() == 2 && words[0].eq_ignore_ascii_case("view") {
            let department = words[1];
            if let Some(employees) = company.get(department) {
                println!("Employees in {}: {:?}", department, employees);
            } else {
                println!("No employees found in {}.", department);
            }

        //  Handling the request to view all departments sorted
        } else if input.eq_ignore_ascii_case("all") {
            for (department, employees) in &company {
                println!("{}: {:?}", department, employees);
            }
        } else {
            println!("Invalid command format.");
        }
    }
}
