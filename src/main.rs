// Error Handling
// To panic! or Not to panic!
// So how do we decide when we should call panic! and when we should return Result ?
// When code panics , there's no way to recover . we could call panic! for any error situation ,
// whether there's a possible way to recover or not or not , but then we're making the decision on
// behalf of the code calling our code that a situation is unrecoverable . when we choose to return
// a Result value , we give the calling code options rather than making the decision for it .
// The calling code could choose the attempt to recover in a way that's appropriate for it's situation,or it could decide
// that an Err value in this case is unrecoverable , so it can call panic! and turn our recoverale error into
// an unrecoverable one .
// Therfore , returning Result is a good default choice when we're defining a function that might
// fail . In rare situations , it's more appropriate to write code that panics instead of returning
// a Result.
fn main() {}
