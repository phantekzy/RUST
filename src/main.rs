// COMMON COLLECTIONS
// Bytes and Scalar Values and Grapheme Clusters
// Another point about UTf-8 is that there are actually three relevant ways to look at strings from
// Rust's prespective : as bytes , scalar values , and grapheme clusters (The closest thing that we
// would call letters).
// If we look at the Hindi word written in the Devanagari script , it is stored as a vector of u8
// That's 18 bytes and is how computers ultimately store this data . If we look at tthem as Unicode
// scalar values , which are Rust's char type is it would look letter by letter
// Unicode scalar values = char type .
// And in this example there are six char vallues here , but the fourth and sixth are not letters :
// they're diacritics that don't make sense on their own . Finally, if we look at them as grapheme
// clusters , we'd get what a person would call the four letters that make up the Hindi word .
// Rust provides different ways of interpreting the raw string data that computers store  so that
// each program can choose the interpretation it needds , no matter what human language the data is
// in .
// A final reason Rust doesn't allow us to index into a String to get a character is that indexing
// operations are expected to alwaus take constant time (0(1)). But isn't possible to guarantee that
// performance with a String, because Rust would have to walk through the contents from the
// beginning to the index to determine how many valid characters there were .
//
// REVIEW THIS PART I WAS SO SLEEPY TO UNDERSTANT THIS CHAPTER
fn main() {}
