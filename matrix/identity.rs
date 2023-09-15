//Check Identity Matrix: Write a function to check if a given matrix is an identity matrix.
fn main(){
    let myvec1: Vec<Vec<i32>> = vec![
        vec![1, 0, 0],
        vec![0, 1, 0],
        vec![0, 0, 1],
    ];

    let rows = myvec1.len();
    let coloums = myvec1[0].len();
    let mut res : bool = true;
   
    for i in 0..rows{
        for j in 0..coloums{
            if i == j{
                if myvec1[i][j] == 1{
                    continue;
                }
                else{
                    res = false;
                }
            }
        }
    }

    if res == true{
        print!("It is identity");
    }
    else{
        print!("It is not identity");
    }
}