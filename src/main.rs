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
// during development . Similarly, panic! is ofter appropriate if we're calling external code that
// However, when failure is expected , it's more appropriate to treturn a Result than to make a
// panic! call . Example inclue a parser being given mal formed data or an HTTP request returning a
// status that indicates you have hit a rate limit . In these cases, returning a Result indicates
// that failure is an expected possibility that the calling code must decide how to handle .
//
// When our code performs operations on values , our code should verify the values are valid first
// and panic if the values aren't valid . This is mostly for safety reasons: attempting to operate
// on invalid data can expose our code to vulnerabilities. This is the main reason the standard
// library will call panic! if we attemtps an out-of-bounds memory access : trying to access memory
// that doesn't belong to the current data structure is a common security problem .

// Main function
fn main() {}
