//THE STACK AND THE HEAP
// OWNERSHIP
// REFRENCES AND BORROWING
// DANGLING REFRENCES
// SLICE TYPE
fn main() {
    // I HAD TO REVIEW LOOPS
    // Consuming the data
    let names = vec!["Lotfi", "Yuro", "King"];
    for name in names {
        println!("{}", name);
    }
    // ERROR
    println!("Total of people : {}", names.len());
}
