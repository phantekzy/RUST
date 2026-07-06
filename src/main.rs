//THE STACK AND THE HEAP
// OWNERSHIP
// REFRENCES AND BORROWING
// DANGLING REFRENCES
// SLICE TYPE
// STRING SLICES
fn main() {
    let s = String::from("Maini Lotfi");
    let maini = &s[0..5];
    let lotfi = &s[6..11];
    println!("{}", maini);
    println!("{}", lotfi);
}
