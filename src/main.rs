// Error Handling
// Examples , Prototype code and Tests :
// When we're writing an example to illustrate some concept , having robust error-handling code in
// the example as well can make the example less clear .
// In examples , it's understood that a call to a method like unwrap that could panic is meant as a
// placeholder for the way we'd want our application hande errors , which can differ based on what
// the rest of our code is doing .
// Similarly , the unwrap and expect methods are very handy when prototyping , before we're ready to
// decide how to handle errors . They leave clear markers in our code for when we're ready to make
// our program more robust.
// If a method call fails in a test , we'd want the whole test to fail , even if that method isn't
// the functionality under test . Because panic! is how a test is marked as a failure , calling
// unwrap or expect is exactly what should happen .
//
// Cases in Which we have more information than the compiler :
// It would also be appropriate to call unwrap when we have some other logic that ensures the Result
// will have an Ok value , but the logic isn't something the compuler understands .
// We'll still have a Result value that we need to hanlde : whatever operation we're calling has the
// possibility of failing in general , even though it's logically impossible in our particular
// situation .
// Main function
fn main() {}
