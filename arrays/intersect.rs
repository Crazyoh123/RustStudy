//Create a program to find the intersection of two arrays (i.e., common elements).

use std::io;

fn main(){
    let array1 : [i32; 5] = [1,2,3,4,5];
    let array2 : [i32; 6] = [1,2,6,7,8,4];
    let mut count  : i32 = 0;
    for i in 0..array1.len(){
        for j in 0..array2.len(){
            if array1[i] == array2[j]{
                count += 1;
            }
        }
    }
    print!("{}",count);
}