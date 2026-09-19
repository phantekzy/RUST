// OPTION TYPE
// Option<T> Enum
// ain function
fn main() {
    // we need to tell Rust what type of Option<T> we have ,
    // because the compiler can't infer the type that
    // the some variant will hold by looking only at None value
    let _absent_number: Option<i32> = None;
    // Some section
    // In some , we know that a value is present and the
    // value is held within the Some
    let _some_number = Some(5);
    let _some_string = Some("a String");

    // I have a question
    // When we have a None value , in some sense , it means the
    // same thing as null : we don't have a valid value .
    // WHY THE HELL WE have to use Option<T> if its null ???
    // THE ANSWER :
    // In short , because Option<T> and T where T can be
    // any type are different Types
    // so it means Option<T> it self and The "T" are diff
    // So the compiler won't let us use an Option<T> value
    // as if it were definitely a valid value
    // Exemple :
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y;
    // We are having this error when we try to add them
    // 1. cannot add `Option<i8>` to `i8`
    //the trait `Add<Option<i8>>` is not implemented for `i8` [E0277]
    //internal_macros.rs:22:9: the following other types implement trait `Add<Rhs>`
    // This error means that Rust does not understand how to add an i8 and an Option<T>
    // They are different types.
}
