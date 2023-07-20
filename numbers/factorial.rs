//Write a program to calculate the factorial of a given number.

use std::io;
fn factorial(n: i32){
    let mut fact = 1;
    if n == 0{
        let j = 0;
        print!("{}",j);
    }
    for i in 1..=n{
        fact = fact * i;
    }
    print!("FACTORIAL OF {} IS {}",n,fact);
}
fn main(){
    let mut str1 = String::new();
    io::stdin().read_line(&mut str1).expect("IT IS NOT TRUE");
    let num:i32 = str1.trim().parse().expect("IT IS NOT TRUE");
    factorial(num);
}