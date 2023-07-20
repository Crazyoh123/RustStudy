//Create a function to find the second smallest element in an array.

use std::io;

fn main(){
    let mut array1 : [i32; 6] = [1,2,3,0,4,5];
    array1.sort();
    print!("{}",array1[2]);
}