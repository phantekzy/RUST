// Control flow
// Repetion with loops
// Returning values from loops
// Looping Through a condition with for
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index != 5 {
        println!("The value is {}", a[index]);
        index += 1;
    }
}
