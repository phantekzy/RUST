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
            ErrorKind::NotFound => match File::create("Hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file : {:?}", e),
            },
            other_error => panic!("Problem opening the file : {:?}", other_error),
        },
    };
}
