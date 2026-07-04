//THE STACK AND THE HEAP
// OWNERSHIP
// REFRENCES AND BORROWING
// DANGLING REFRENCES
// SLICE TYPE
fn main() {
    // I HAD TO REVIEW LOOPS
    let mut numbers = vec![1, 2, 3];
    for num in &mut numbers {
        *num *= 2;
        println!("{}", num);
    }
}
