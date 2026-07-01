//THE STACK AND THE HEAP
// OWNERSHIP
// REFRENCES AND BORROWING
fn main() {
    let s1 = String::from("REFERENCES"); // s1 enters to the scope
    let len = calculate_length(&s1); // A reference is like a pointer in C
    println!("The length of '{}' is {}", s1, len)
}

fn calculate_length(s: &String) -> usize {
    // the function takes a String reference
    s.len()
}
