use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn trash() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Password is {}", secret_number);

    println!("Input the Password");
    let mut pass = String::new();
    io::stdin()
        .read_line(&mut pass)
        .expect("Can't crack the Password");

    let pass: u32 = pass.trim().parse().expect("Please type a number");

    match pass.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Bigger"),
        Ordering::Equal => println!("Equal"),
    }
}
