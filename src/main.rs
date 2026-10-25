use std::fs::File;

// Error Handling
// Shortucts for Panic on Error : unwrap and expect
// Using match works well enough , but it can be a bit verbose and doesn't always communicate intent
// well . The Result<T,E> type has many helper methods defined on it to do various tasks . One of
// those methods , called unwrap , is a shortcut method that is implemented just like the match
// expression we wrote in the last example . If the Result value is OK variant , unwrap will return
// the value inside the OK. If the Result is the Err variant , unwrap will call the panic! macro for
// us .
fn main() {
    // Here is an example of wunwrap in action :
    let f = File::open("hello.txt").unwrap();
}
