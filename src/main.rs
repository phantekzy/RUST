//THE STACK AND THE HEAP
// OWNERSHIP
// String type
// Why strings can be mutated but not litterals ?
fn main() {
    let mut s = String::from("hello");
    s.push_str(", phantekzy");
    println!("{}", s);
}
