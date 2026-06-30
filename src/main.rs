//THE STACK AND THE HEAP
// OWNERSHIP
// Return values and scope
// Returning multiple values using tuples
fn main() {
    let s1 = String::from("Multiple");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns a length of a String
    (s, length)
}
