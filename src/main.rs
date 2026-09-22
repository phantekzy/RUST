// Patterns That Bind to Values
// Another useful feature of match arms is that they cn bind to the
// parts of the values that match the pattern.
// This is how we can extract values out of enum variants .
//
// From 1999 through 2008 , the US minted quarters with different designs for each of the 50 states
// on one side . No other coins got state designs , so only quarters have this extra value .
// We can add this information to our enum by changing the Quarters variant include a UsState value
// stored inside of it

#[derive(Debug)] // So we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    NewYork,
    California,
    // -- snip --
}
// A Coin enum in which the Quarter variant aslo holds a UsState value
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
// Let's imagine that a friend of ours is trying to collect all 50 state quarters.
// while we sort our loose change by coin type , we'll also call out the name of the state
// assicuated with each quarter so if it's one our friend doesn't have , the can add it to their
// collection
//
// In the match expression for this  code , we add a variable called state to the pattern that
// matches values of the variant Coin::Quarter.
// when a Coin::Quarter matches , the state variable will bind to the value of that quarter's state
// Main function
fn main() {}
