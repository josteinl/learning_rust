use std::io;

fn fibonacci(n: u128) -> u128 {
    if n == 0 {0}
    else if n == 1 {1}
    else { fibonacci(n - 1) + fibonacci(n - 2) }
}

fn main() {
    println!("Let's find the Fibonacci number N:");

    println!("Please enter the number N:");
    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    let n: u128 = n.trim().parse().expect("Please enter a number!");

    let result= fibonacci(n);
    println!("The {n}th fibonacci number is: {result}");


}
