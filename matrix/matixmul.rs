//Matrix Multiplication: Write a function to multiply two matrices.

fn matrix_multiplication(matrix1: Vec<Vec<i32>>, matrix2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result_matrix: Vec<Vec<i32>> = vec![vec![0; 2]; 2];

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                result_matrix[i][j] += matrix1[i][k] * matrix2[k][j];
            }
        }
    }

    result_matrix
}

fn main() {
    let matrix1: Vec<Vec<i32>> = vec![vec![10, 20], vec![30, 40]];
    let matrix2: Vec<Vec<i32>> = vec![vec![10, 20], vec![30, 40]];

    let result_matrix = matrix_multiplication(matrix1, matrix2);
    println!("{:?}", result_matrix);
}
