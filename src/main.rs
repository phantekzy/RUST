// COMMON COLLECTIONS
// Reading Elements of Vectors
// Now that we know how to create , update , and destroy vectors , knowing how to read their
// contents is a good next step .
// There are two ways to reference a value stored in a vector .
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    // Using indexing syntax
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    // using the get method to access an item in a vector
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }
}
