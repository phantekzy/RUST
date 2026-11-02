// Error Handling
// Creating Custom types for validation :
// Let's take the idea of using Rust's type system to ensure we have a valid value on step further
// and look at creating a custom type for validation.
// Lets use the guessing game example , which our code asked the user to guess a number between 1
// and 100. whe never validated that the user's guess was between those numbers before checkingh it
// against our secret number; We only validated that the guess was positive . In this case , the
// consequences were not very dire : Our output of "too high" and "Too low" would still be correct .
// But it would be useful enhancement to guide the user toward valid guesses and have different
// behavior when a user guesses a number that's out of the range  versus when a user types , for
// example, letters instead .
// One way to do this would be to parse the guess as an i32 instead of only a u32 to allow
// potentially negative numbers, and then add a check for the number being in range :

// Main function
// This example shows one way to define Guess type that will only create an instance of Guess if
// the new function recieves a value between 1 to 100 :
pub struct Guess {
    value: i32,
}
fn main() {
    // -- snip--
    loop {
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
    if guess < 1 || guess > 100 {
        println!("The secret number will be between 1 and 100.");
        continue;
    }
    match guess.cmp(&secret_number) {
        // --snip--
    }
}
