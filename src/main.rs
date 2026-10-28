// Error Handling
// To panic! or Not to panic!
// So how do we decide when we should call panic! and when we should return Result ?
// When code panics , there's no way to recover . we could call panic! for any error situation ,
// whether there's a possible way to recover or not or not , but then we're making the decision on
// behalf of the code calling our code that a situation is unrecoverable . Hwen we choose to return
// a Result alue , we give the calling code options rather than making the decision for it .
fn main() {}
