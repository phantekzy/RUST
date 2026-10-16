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
}
