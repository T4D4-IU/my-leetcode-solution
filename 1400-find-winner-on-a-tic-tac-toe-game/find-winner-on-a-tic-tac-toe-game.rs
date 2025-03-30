impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        let mut grid = vec![vec![" "; 3]; 3];
        let mut winer = "Draw";
        let mut moves_left = 9;

        for (i, move_) in moves.iter().enumerate() {
            let row = move_[0] as usize;
            let col = move_[1] as usize;
            let symbol = if i % 2 == 0 { "A" } else { "B" };
            grid[row][col] = symbol;
            moves_left -= 1;
        }
        
        for i in 0..3 {
            if grid[i][0] == grid[i][1] && grid[i][1] == grid[i][2] {
                winer = grid[i][0];
            }
            if grid[0][i] == grid[1][i] && grid[1][i] == grid[2][i] {
                winer = grid[0][i];
            }
        }
        if grid[0][0] == grid[1][1] && grid[1][1] == grid[2][2] {
            winer = grid[0][0];
        } else if grid[0][2] == grid[1][1] && grid[1][1] == grid[2][0] {
            winer = grid[1][1];
        }
        if winer == "Draw" && moves_left > 0 {
            winer = "Pending";
        } else if winer == " " {
            winer = "Pending";
        }
        winer.to_string()
    }
}