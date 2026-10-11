// COMMON COLLECTIONS
// Reading Elements of Vectors

fn main() {
    // When the program has a valid reference , the borrow checker enforces the ownership and
    // borrowing rules to ensure this reference and any other references to the contents of the
    // vector remain valid .
    // Recall the rule that states you can't have mutable and immutable references in the same
    // scope.
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // Attempting to add an element to a vector while holding a reference to an item
    v.push(6);
    println!("The first element is : {}", first)
}
