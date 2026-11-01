// Error Handling
// Guidelines for Error Handling
// It's advisable to have our code panic when it's possible that our code could end up in a bad
// state . In this context , a bad state in when some assuption, guarantee , contract , or invariant
// has been broken , such as when invalid values , contradictory values , or missing values are
// passed to our code plus one or more of the following :
// - The bad state is not somthing that's expected to happen occasionally .
// - Our code after this point needs to rely on not being in this bad state .
// - There is not a good way to encode this information in the types we use .
// If someone calls our code and passes in values that don't make sense , the best choice might be
// to call panic! and alert the person using our library to the bug in their code so they can fix in
// during development .

// Main function
fn main() {}
