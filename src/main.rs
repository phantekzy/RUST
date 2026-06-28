//THE STACK AND THE HEAP
// OWNERSHIP
// String type
// Why strings can be mutated but not litterals ?
fn main() {
    let mut s = String::from("hello");
    s.push_str(", phantekzy"); // push_str appends a litteral to a String
    println!("{}", s); // This will print 'phantekzy'
}
