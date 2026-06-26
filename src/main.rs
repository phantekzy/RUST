// Control flow
// Repetion with loops
// Returning values from loops
fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("the result is {}", result)
}
