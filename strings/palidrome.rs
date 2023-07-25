//Write a function to check if a given string is a palindrome (reads the same forwards and backwards).

fn is_palindrome(s: &str) -> bool {
    let mut start: usize = 0;
    let mut end: usize = s.len() - 1;
    
    while start < end {
        if s.chars().nth(start) != s.chars().nth(end) {
            return false; 
        }
        start += 1;
        end -= 1;
    }
    true
}

fn main() {
    let my_string = String::from("HEH");
    
    if is_palindrome(&my_string) {
        println!("IT IS A PALINDROME!");
    } else {
        println!("IT IS NOT A PALINDROME!");
    }
}
