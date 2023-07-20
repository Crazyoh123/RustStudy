//Write a program to find the least common multiple (LCM) of two numbers.

use std::io;

fn gcdofnum(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcdofnum(b, a % b)
    }
}

fn lcmoftwonum(a: i32, b: i32) -> i32 {
    let gcd = gcdofnum(a, b);
    (a * b) / gcd
}

fn main() {
    let mut str1 = String::new();
    let mut str2 = String::new();
    io::stdin().read_line(&mut str1).expect("Failed to read input");
    io::stdin().read_line(&mut str2).expect("Failed to read input");
    let num1: i32 = str1.trim().parse().expect("Invalid number");
    let num2: i32 = str2.trim().parse().expect("Invalid number");
    let res = lcmoftwonum(num1, num2);
    println!("{}", res);
}
