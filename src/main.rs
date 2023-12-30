use std::io;

fn main() {
    println!("Enter a number to get the nth fibonacci number:");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    let n: i32 = input.trim().parse().expect("Please enter a valid number.");
    let fib = fibonacci(n);
    println!("The {n}th fibonacci number is {fib}");
}

fn fibonacci(n: i32) -> i32 {
    if n == 1 {
        return n;
    } else {
        return n + fibonacci(n - 1);
    }
}
