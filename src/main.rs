// Matches Are Exhaustive
// There is another aspect of match , let us consider this version of our plus_one functionthat has
// a bug and won't compile :
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}

//
// Main function
fn main() {}
