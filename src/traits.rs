// What is a Trait ?
// Imagine we have two completely different structs ; a Dog and a Robot. Both can make a sound, even
// tough they are completely different things.
//
// We can define a speak trait that promises any type using it will have a say_hello method.
//
// 1. Define the trait  (the"contract") :
trait Speak {
    fn say_hello(&self) -> String;
}
struct Dog {
    name: String,
}

struct Robot {
    model_id: u32,
}
