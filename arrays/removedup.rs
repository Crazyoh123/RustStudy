//Implement a program to remove duplicate elements from an array.

use std::io;

fn main(){
    let mut res : bool = false;
    let array1 : [i32 ; 5] = [1,2,3,1,5];
    for i in 0..array1.len() - 1{
        for j in 1..array1.len() - 1{
            if array1[i] == array1[j]{
                res = true;
                break;
            }
        }
    }
    if res {
        print!("IT  CONTAN DUPLICATES!");
    }
    else{
        print!("IT DONT HAVE  DUPLICATES");
    }

}