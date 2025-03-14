impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let rows = grid.len();
        let cols = grid[0].len();
        let total_elements = rows * cols;
        let k = (k as usize) % total_elements;
        
        if k == 0 {
            return grid;
        }
        
        let mut result: Vec<Vec<i32>> = vec![vec![0; cols]; rows];
        
        for i in 0..rows {
            for j in 0..cols {
                let new_index = (i * cols + j + k) % total_elements;
                let new_row = new_index / cols;
                let new_col = new_index % cols;
                result[new_row][new_col] = grid[i][j];
            }
        }
        
        result
    }
}