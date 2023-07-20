//Write a function to check if an array is sorted in ascending order.

use std::io;
fn main(){
    let mut res : bool = true;
    let array1 : [i32; 5] = [1, 2, 3, 4, 5];
    for i in 0..array1.len() - 1{
        if array1[i] > array1[i + 1]{
            res = false;
            break;
        }
    }
    
    if res{
        print!("Sorted");
    } 
    else{
        print!("Not Sorted!")
    }
}