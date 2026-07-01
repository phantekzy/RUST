//THE STACK AND THE HEAP
// OWNERSHIP
// REFRENCES AND BORROWING
fn main() {
    let s1 = String::from("REFERENCES"); // s1 enters to the scope
    let len = calculate_length(&s1); // A reference is like a pointer in C
    println!("The length of '{}' is {}", s1, len);
}
fn calculate_length(s: &String) -> usize {
    // s is a reference to a string
    s.len()
}
// here , s goes out of the scope , but because it does not have
// ownership of it refers to , so nothing happens
