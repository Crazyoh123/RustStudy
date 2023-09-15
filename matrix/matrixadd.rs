// Function to add two matrices of the same size
fn matrix_addition(matrix1: &[Vec<i32>], matrix2: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let rows = matrix1.len();
    let cols = matrix1[0].len(); // Assuming both matrices have the same number of columns

    let mut result_matrix = vec![vec![0; cols]; rows];

    for i in 0..rows {
        for j in 0..cols {
            result_matrix[i][j] = matrix1[i][j] + matrix2[i][j];
        }
    }

    result_matrix
}

fn main() {
    let matrix1: Vec<Vec<i32>> = vec![
        vec![1, 2],
        vec![3, 4],
    ];

    let matrix2: Vec<Vec<i32>> = vec![
        vec![5, 6],
        vec![7, 8],
    ];

    let result_matrix = matrix_addition(&matrix1, &matrix2);

    // Print the result matrix
    for row in &result_matrix {
        println!("{:?}", row);
    }
}
