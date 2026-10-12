// COMMON COLLECTIONS
// Updating a String
// Concatenation with the + Operator or the format! Macro
// Main function
fn main() {
    // Often we'll want to combine two existing strings. one way is to use the + operator :
    let s1 = String::from("hello, ");
    let s2 = String::from("world!");
    // Using the + operator to combine two string values into a new String value
    let s3 = s1 + &s2; // note that s1 has been moved here and can no longer be used 
    println!("{}", s3);
    // The string s3 will contain Hello, world! as a result of this coe . Then reason s1 is no
    // longer valid after the addition and the reason we used a reference to s2 has to do wit h the
    // signature of the method that gets called when we use the + operator .
    // The + operator uses the add method , whose signature looks like this :
    // fn add(self, s: &str) -> String {}
    // S2 has an &, meaaning that we're adding a refernece of the second string to the first string
    // because of the s parameter in the add function : we can only add a &str to a String ; we
    // can't add two String valuees together . but wait the type of &s2 is &String ,not &str , as
    // specified in the second parameter to add .
    // The reason we're able to use &s2 in the call to add is that the compiler can "coerce" the
    // &String argument into a &str. When we call the add method, Rust uses a deref coercion, which
    // here turns &s2 into &s2[..] . we'll talk about deref coercion later .
    // Because add does not take ownership of the s parameter , s2 will still be a valid String
    // after this operation.
    // Second we can see the signature that add takes ownership of self , because self does not have
    // an &. This means s1 will me moved into add call and no longer be valid after that.
    // So although let s3 = s1+&s2; looks like it will copy both strings and create a new one , this
    // statement actualy takes ownership of s1 , appends a copy of the contents of s2 , and then
    // returns ownership of the result . In other words , it looks like it's making a lot of copies
    // but isn't ; the implementation is more efficient than copying .
}
