//THE STACK AND THE HEAP
// OWNERSHIP
// REFRENCES AND BORROWING
// DANGLING REFRENCES
// SLICE TYPE
// STRING SLICES
fn main() {
    // Variables
    let s = String::from("Maini Lotfi");
    let p = String::from("Testing");
    // String slices
    let maini = &s[0..5];
    let lotfi = &s[6..11];
    let test = &p[0..7];
    // Print
    println!("{}", maini);
    println!("{}", lotfi);
    println!("{}", test);
}
