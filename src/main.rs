use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Password crack");
    // Password generated
    let password = rand::thread_rng().gen_range(0, 101);
    // OPTIONAL : show the generated Password
    //println!("GENERATED PASSWORD : {}", password);
    loop {
        println!("Enter the right password");
        // Instance of the string
        let mut guess = String::new();
        // Line input reading
        io::stdin()
            .read_line(&mut guess)
            .expect("Impossible to read the line ");
        // Shadowing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // Comparaison between values
        match guess.cmp(&password) {
            // Equal case
            Ordering::Equal => {
                println!("The Password is correct");
                break;
            }
            Ordering::Less => println!("Password charachters are less"),
            Ordering::Greater => println!("Password characters are more"),
        }
    }
}
