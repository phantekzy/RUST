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

use rand::Rng;

// Defining the custom type to protect the logic
pub struct Guess {
    value: i32,
}
impl Guess {
    // Constructor : only build a Guess if it's valid
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value)
        }
        Guess { value }
    }
    // Getter : allows safe , read only access to the secret number
    pub fn value(&self) -> i32 {
        self.value
    }
}

// Main function
fn main() {
    println!("Welcom to the guessing game");
    // Generate a random secret number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Please input your guess (1-100)");
        let mut input = String::new();
    }
}
