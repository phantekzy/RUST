// Error Handling
// Matching on Different Errors :
// The code will panic! no matter why File::open failed. What we want to do instead is take
// different actions for different failure reasons : if File::open failed because the file doesn't
// exist , we want to create the file and return the handle to the new file .
// Main function
fn main() {}
