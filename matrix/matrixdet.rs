//Matrix Determinant: Write a function to find the determinant of a square matrix.


fn determinant(matrix: &Vec<Vec<i32>>) -> i32 {
    assert_eq!(matrix.len(), 3); // Check that it's a 3x3 matrix
    assert_eq!(matrix[0].len(), 3);
    
    let a = matrix[0][0];
    let b = matrix[0][1];
    let c = matrix[0][2];
    let d = matrix[1][0];
    let e = matrix[1][1];
    let f = matrix[1][2];
    let g = matrix[2][0];
    let h = matrix[2][1];
    let i = matrix[2][2];

    a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g)
}

fn main() {
    let myvec1: Vec<Vec<i32>> = vec![
        vec![1, -2, 5],
        vec![3, 4, 6],
        vec![90, 4, 3],
    ];

    let det = determinant(&myvec1);
    println!("Determinant: {}", det);
}
