//Write a program to find the sum of all numbers from 1 to N.

use std::io;

fn calculate_sum(n: u32) -> u32 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    sum
}

fn main() {
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read input.");

    let num: u32 = num.trim().parse().expect("Invalid input.");
    let sum = calculate_sum(num);

    println!("THE SUM OF {} IS {}",num,sum);
}
