//Write a program to find the maximum and minimum element in an array.

use std::io;

fn main(){
    let array1 : [i32; 5] = [1,2,3,4,5];
    let mut max = array1[0];
    for i in 0..array1.len(){
        if max < array1[i] {
            max = array1[i]
        }
    }
    print!("The Maximum is: {}",max);
}