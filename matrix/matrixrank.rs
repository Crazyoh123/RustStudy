//Matrix Rank: Write a function to find the rank of a matrix.

fn find_rank(matrix: &mut Vec<Vec<f64>>) -> usize {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut rank = 0;
    let mut lead = 0;

    for row in 0..rows {
        if lead >= cols {
            break;
        }

        let mut i = row;
        while matrix[i][lead] == 0.0 {
            i += 1;
            if i == rows {
                i = row;
                lead += 1;
                if cols == lead {
                    return rank;
                }
            }
        }

        matrix.swap(i, row);

        let lv = matrix[row][lead];
        for j in 0..cols {
            matrix[row][j] /= lv;
        }

        for i in 0..rows {
            if i != row {
                let lv = matrix[i][lead];
                for j in 0..cols {
                    matrix[i][j] -= lv * matrix[row][j];
                }
            }
        }

        lead += 1;
        rank += 1;
    }

    rank
}

fn main() {
    let mut matrix: Vec<Vec<f64>> = vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0],
    ];

    let rank = find_rank(&mut matrix);
    println!("Rank of the matrix: {}", rank);
}
