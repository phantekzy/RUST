// Patterns That Bind to Values
// Another useful feature of match arms is that they cn bind to the
// parts of the values that match the pattern.
// This is how we can extract values out of enum variants .
enum UsState {
    Alabama,
    Alaska,
    NewYork,
    California,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
// Main function
fn main() {}
