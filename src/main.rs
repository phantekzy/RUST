use std::collections::HashMap;

// COMMON COLLECTIONS
// Hash Maps and Ownership :
// For types that implement the Copy trait , sucj as i32 , the values are copied into the hash map .
// For owned values such String , the values will be moved and the hash map will be the owner of
// those values :
// Main function
fn main() {
    let filed_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(filed_name, field_value);
    // filed_name and field_value are invalid at this point !
    // We aren't able to use the varables filed_name and field_value after the have been moved into
    // the  hash map with the call to insert.
    // If we insert references to values into the hash map , the values won't be moved into the
    // HashMap . The values that the references point to must be valid for at least as long as the
    // hash map is valid .
}
