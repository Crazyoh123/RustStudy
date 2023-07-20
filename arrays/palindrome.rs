//Write a function to check if an array is a palindrome (i.e., reads the same forwards and backwards).\


use std::io;

fn main() {
    let mut res: bool = true;
    let array1: [i32; 5] = [1, 2, 3, 2, 1];
    let mut start: usize = 0;
    let mut end: usize = array1.len() - 1;

    while start < end {
        if array1[start] != array1[end] {
            res = false;
            break;
        }
        start += 1;
        end -= 1;
    }

    if res {
        print!("PALINDROME");
    } else {
        print!("NOT PALINDROME");
    }
}
