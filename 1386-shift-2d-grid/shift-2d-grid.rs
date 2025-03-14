impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut grid = grid;
        let mut result: Vec<Vec<i32>> = vec![vec![0; cols]; rows];
        if k == 0 {
            return grid;
        }

        for _ in 0..k {
            for i in 0..rows {
                for j in 0..cols {
                    if j + 1 < cols {
                        result[i][j + 1] = grid[i][j];
                    } else if i + 1 < rows {
                        result[i + 1][0] = grid[i][j];
                    } else {
                        result[0][0] = grid[i][j];
                    }
                }
            }
            grid = result.clone();
        }
        result
    }
}