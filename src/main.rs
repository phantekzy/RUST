// OPTION TYPE
// The option Enum and its Advantages over Null Values
// In the previous section , we looked at thow the IpAddr enum let us use Rust's type system
// to encode more information than just the data into our program.
// This section explores a case study of "Option", which is another enum defined by
// the Standard Library.
//
// The option type is used in many places because it encodes the very common scenario in which value
// could be something or it could be nothing.
// Expressing this concept in terms of tye type system means the compiler can check wheter you've
// handled all the cases you should be handling ; this functionality can prevent bugs that are
// extremly common in other programming languages.
//
// Programming languages design is ofter thought of in terms of which features you include ,
// but the features you exclude are important too.
// Rust doesn't have the null feature that many other languages have . Null is a value that means
// there is no value there
// In languages with null , variables can always be in one of two states : null or not-null.
//
// Rust does not have nulls, but it does have an enum that can encode the concept of a value
// being present or absent
// This enum is Option<T> , and it is defined by the standard library as follows :
enum Option<T> {
    Some(T),
    None,
}
// The Option<T> enum is very usefull that it's even included in the prelude ;
// We don't need to bring it into scope explicitly.
//
// The variants of Option :
// Some and None
// We can use Some and None directly without the Option:: prefix .
// Option enum can hold one piece of data of any type
// Main function
fn main() {
    let some_number = Some(5);
    let some_string = Some("a String");
}
