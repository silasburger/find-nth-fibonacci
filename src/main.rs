use std::io;
fn find_fib(num: i64) -> i64 {
    if num == 1 {
        1
    } else if num == 0 {
        0
    } else {
        find_fib(num - 1) + find_fib(num - 2)
    }
}

fn main() {
    let mut input_string = String::new();

    println!("Enter a number");

    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");

    let num: i64 = input_string.trim().parse().unwrap();
    println!("The fibonacci number at {} is {}", num, find_fib(num));
}