// Matches Are Exhaustive
// There is another aspect of match , let us consider this version of our plus_one functionthat has
// a bug and won't compile :
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // I didn't handle the None case , so this code will cause a bug
        // If we try to compile the code , we will get an error
        Some(i) => Some(i + 1),
    }
}
// Rust knows that we didn't cover every possible case and even knows wihich pattern i forgot !
// Matches in Rust are exhaustive : we must exhaust every last possibility in order for the code to
// be valid .
// Especially in the case of Option<T> , when Rust prevents us from forgetting to explicitly handle
// the None casse , it protects us from assuming that we have a value when we might have null

// Main function
fn main() {}
