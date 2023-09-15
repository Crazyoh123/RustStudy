//Upper Triangular Matrix: Write a function to check if a given matrix is upper triangular.

fn is_upper_triangular(matrix: &[Vec<i32>]) -> bool {
    let rows = matrix.len();
    let cols = matrix[0].len();

    if rows != cols {
        return false; 
    }

    for i in 1..rows {
        for j in 0..i {
            if matrix[i][j] != 0 {
                return false; 
            }
        }
    }

    true
}

fn main() {
    let matrix1: Vec<Vec<i32>> = vec![
        vec![1, 2, 3],
        vec![0, 4, 5],
        vec![0, 0, 6],
    ];
    if is_upper_triangular(&matrix1) {
        println!("Matrix 1 is upper triangular.");
    } else {
        println!("Matrix 1 is not upper triangular.");
    }
}