//THE STACK AND THE HEAP
// OWNERSHIP
// REFRENCES AND BORROWING
fn main() {
    let s1 = String::from("REFERENCES");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len)
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
