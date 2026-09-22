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
}
// A Coin enum in which the Quarter variant aslo holds a UsState value
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
// Main function
fn main() {}
