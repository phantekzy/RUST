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
    // because of the s parameter in the add function : we can only add a &str to a String ;
}
