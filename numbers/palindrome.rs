//Create a function to determine if a given number is a palindrome.
use std::io;
fn palindrome(a:i32){
    let mut k = a;
    let mut rev = 0;
    while k!=0{
        rev = (rev*10) + k%10;
        k = k / 10;
    }
    if a == rev{
        print!("IT IS PALINDROME");
        return;
    }
    print!("IT IS NOT PALINDROME");
}
fn main(){
    let mut str1 = String::new();
    io::stdin().read_line(&mut str1).expect("IT CANNOT ACCEPT");
    let num1 : i32 = str1.trim().parse().expect("IT CANNOT PARSE");
    palindrome(num1);
}