fn main() {
    let mut x = 67;
    println!("The value of x is : {}", x);
    x = 69;
    println!("The value of x is :{}", x);
    const MAX_POINTS: u32 = 100_000;
    println!("{}", MAX_POINTS);
    let x = x + 1;
    let x = x * 2;
    println!("Shadowing the x : {}", x);
}
