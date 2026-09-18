// OPTION TYPE
// Option<T> Enum
// ain function
fn main() {
    // Some section
    let some_number = Some(5);
    let some_string = Some("a String");
    // we need to tell Rust what type of Option<T> we have ,
    // because the compiler can't infer the type that
    // the some variant will hold by looking only at None value
    let absent_number: Option<i32> = None;
}
