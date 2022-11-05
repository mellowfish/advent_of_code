use std::io;

fn main() {
    let index = read_number();
    let value = fib(index);

    println!("{value}");
}

fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1..=2 => 1,
        _ => fib(n - 1) + fib(n - 2)
    }
}

fn read_number() -> u32 {
    println!("Enter a number:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().parse().expect("Failed to parse number")
}
