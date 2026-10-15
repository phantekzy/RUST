// COMMON COLLECTIONS
// Slicing Strings
// Indexing into a string is often a bad idea because it's not cleat what the returh type of the
// string Indexing operation should be : a byte value , a character , a grapheme cluster , or a
// string slice .
// Therefore , Rust asks you to be more specific if you really need to use indices to create string
// slices .
// To be more specific in your indexing and ubducate tgat you wabt a string slice , rhater than
// indexing using [] with a single number m you can use [] with a range to create a string slice
// containing particular bytes :
// Main function
fn main() {
    let hello = "hello";
    let s = &hello[0..1];
    println!("{}", s);
    // Here , will be a &str that contains the first 3 bytes of the string , Earlier we mentioned t
    // hat each of there characters was 2 bytes , or 1 if its english etc
    // If it was in hindi we said that each of its characters was 2 bytes , if we do a range of
    // [0..1] , the compiler would panic
    // We shoud use ranges to create string slices with caution , because doing so can crash our
    // program .
}
