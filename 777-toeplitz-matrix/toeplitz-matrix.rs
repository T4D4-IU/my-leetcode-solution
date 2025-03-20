impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let rows = matrix.len();
        let cols = if rows > 0 {matrix[0].len()} else { 0 };

        for i in 1..rows {
            for j in 1..cols {
                if matrix[i][j] != matrix[i - 1][j - 1] {
                    return false;
                }
            }
        }
        true
    }
}