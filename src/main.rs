//THE STACK AND THE HEAP
// OWNERSHIP
// REFRENCES AND BORROWING
fn main() {
    let s = String::from("BORROWING");
    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str("a value");
}
