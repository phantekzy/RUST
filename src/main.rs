// COMMON COLLECTIONS
// Bytes and Scalar Values and Grapheme Clusters
// Another point about UTf-8 is that there are actually three relevant ways to look at strings from
// Rust's prespective : as bytes , scalar values , and grapheme clusters (The closest thing that we
// would call letters).
// If we look at the Hindi word written in the Devanagari script , it is stored as a vector of u8
// That's 18 bytes and is how computers ultimately store this data . If we look at tthem as Unicode
// scalar values , which are Rust's char type is it would look letter by letter
fn main() {}
