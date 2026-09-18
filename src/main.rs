// OPTION TYPE
// Option<T> Enum
// ain function
fn main() {
    // we need to tell Rust what type of Option<T> we have ,
    // because the compiler can't infer the type that
    // the some variant will hold by looking only at None value
    let absent_number: Option<i32> = None;
    // Some section
    // In some , we know that a value is present and the
    // value is held within the Some
    let some_number = Some(5);
    let some_string = Some("a String");

    // I have a question
    // When we have a None value , in some sense , it means the
    // same thing as null : we don't have a valid value .
    // WHY THE HELL WE have to use Option<T> if its null ???
}
