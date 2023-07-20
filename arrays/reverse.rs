//Implement a function to reverse the elements of an array.

use std::io;

fn main(){
    let array1 : [i32; 5] = [1, 2, 3, 4, 5];
    let length = array1.len();
    for i in (0..length).rev(){
        print!("{} ",array1[i])
    }
}