//Write a function to check if a number is a perfect square or not.
use std::io;

fn perfectsqr(n: i32){
    for i in 0..=n{
        if (i*i) == n{
            print!("YES");
            return;
        }
    }
    print!("NO");
}
fn main(){
    let mut str1 = String::new();
    io::stdin().read_line(&mut str1).expect("THIS IS NOT FAIR");
    let number : i32 = str1.trim().parse().expect("THIS IS NOT FAIR");
    perfectsqr(number);
}