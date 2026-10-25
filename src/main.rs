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
    // Another method , expect , which is similar to unwrap , lets us also choose the panic! error
    // message . Using expect instead of unwrap and providing food error messages can convey your
    // intent and make tracking down the source of a panic easier .
    // The syntax of expect looks like this :
    let f = File::open("hello.txt").expect("Failed to open hello.txt ");
    // We use expect in the same way as unwrap : to return the file handle or call the panic! macro
    // . the error message used by expect in its call to panic! will be the parameter that we pass
    // to expect , rahter than the default panic! message that unwrap uses  :
    // thread 'main' (504954) panicked at src/main.rs:16:37:
    // Failed to open hello.txt : Os { code: 2, kind: NotFound, message: "No such file or directory" }
    // Because this error starts with the text we specified "Failed to open hello.txt" , it will be
    // easier to find where in the code this error message is comming from .
}
