use std::io;

fn main(){
    let array1 : [i32; 5] = [1,2,3,4,5];
    let mut sum = 0;
    for i in 0..array1.len(){
        sum += array1[i];
    }
    print!("{}",sum);
}