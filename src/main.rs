//THE STACK AND THE HEAP
// OWNERSHIP
// REFRENCES AND BORROWING
// DANGLING REFRENCES
// SLICE TYPE
fn main() {
    // I HAD TO REVIEW LOOPS
    let names = vec!["Lotfi", "Yuro", "King"];
    for name in &names {
        println!("{}", name);
    }
    println!("Total of people : {}", names.len());
}
