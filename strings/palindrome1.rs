
//Write a program to check if a string is a valid palindrome after removing non-alphanumeric characters and ignoring case.

use std::io;

fn main() {
    let input_string = String::from("A man, a plan, a canal, Panama!");

    let cleaned_string: String = input_string
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    let is_palindrome = is_valid_palindrome(&cleaned_string);

    if is_palindrome {
        print!("The input is a valid palindrome.");
    } else {
        print!("The input is not a valid palindrome.");
    }
}

fn is_valid_palindrome(s: &str) -> bool {
    let mut left = 0;
    let mut right = s.len() - 1;

    while left < right {
        if s.chars().nth(left) != s.chars().nth(right) {
            return false;
        }
        left += 1;
        right -= 1;
    }

    true
}
