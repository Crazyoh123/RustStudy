//Implement a function to check if a given number is prime or not

use std::io;

fn primeornot(n: i32){
   for i in 2..n{
       if n%i == 0{
         print!("{} IT IS NOT PRIME NUMBER!",n);
         return;
       }
   }
   print!("{} IT IS PRIME NUMBER!!",n);
}
fn main(){
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("IT IS NOT NUMBER");
    let k:i32 = num.trim().parse().expect("IT IS NOT NUMBER");
    primeornot(k);
}