use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(0, 101);
    println!("the secret number is {}", secret_number);

    loop {
        println!("Enter the number");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Impossible to read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Small"),
            Ordering::Equal => {
                println!("You guessed the right number");
                break;
            }
            Ordering::Greater => println!("Greater"),
        }
    }
}
