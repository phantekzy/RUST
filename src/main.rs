use std::{fs::File, io::ErrorKind};

// Error Handling
// Matching on Different Errors :
// The code will panic! no matter why File::open failed. What we want to do instead is take
// different actions for different failure reasons : if File::open failed because the file doesn't
// exist , we want to create the file and return the handle to the new file .
// If File::open failed for any other reason for example , because we didn't have permission to open
// the file we still want the code to panic! in the same way as it did before .
// Main function
fn main() {
    let f = File::open("hello.txt");
    // Handling different kinds of errors in different ways
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file : {:?}", e),
            },
            other_error => panic!("Problem opening the file {:?}", other_error),
        },
    };
    // The type of the value that File::open returs inside the Err variant is io::Error , which is a
    // struct provided by the standard library . This struct has a method kind that we can call to
    // get an io::ErrorKind value . The enum io::ErrorKind is provided by the standard library and
    // has variants representing the different kinds of errors that might reust from an io operation
    // The vairant we want to use is ErrorKind::NotFound , which indicates the file we're trying to
    // open doen't exist yet . Se we match on f, but we also have an inner match on error.kind().
    //
    // The condition we want to check in the inner match is whether the value returned by
    // error.kind() is the NotFound variant of the ErrorKind enum . If it is , we try to create the
    // file with File::create.
}
