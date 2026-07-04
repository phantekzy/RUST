//THE STACK AND THE HEAP
// OWNERSHIP
// REFRENCES AND BORROWING
// DANGLING REFRENCES
fn main() {
    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);
}
// The sollution is to return the String directly and move Ownership
fn dangle() -> String {
    // dangle returns a reference to a String
    let s = String::from("hello");
    s // we return a reference to a String
} // s goes out of the scope , and is dropped . it's memory goes away
