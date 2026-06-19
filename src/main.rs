fn main() {
    let mut x = 67;
    println!("The value of x is : {}", x);
    x = 5;
    println!("The value of x is :{}", x);
    const MAX_POINTS: u32 = 100_000;
    println!("{}", MAX_POINTS);
    // SHADOWING
    let x = x + 1;
    let x = x * 2;
    println!("{}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}", spaces);
}
