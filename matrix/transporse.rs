//Transpose Matrix: Write a function to find the transpose of a given matrix.

fn main(){
    let  myvec1: Vec<Vec<i32>> = vec![
        vec![1,2,3],
        vec![3,4,5],
        vec![6,7,8],
    ];
    let rows = myvec1.len();
    let cols = myvec1[0].len();

    let mut result_matrix = vec![vec![0; cols]; rows];

    for i in 0..rows{
        for j in 0..cols{
            result_matrix[i][j] = myvec1[j][i];
        }
    }
    
    for row in &result_matrix {
        println!("{:?}", row);
    }
}
