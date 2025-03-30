impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        // 3x3のグリッドを初期化（すべて空白で開始）
        let mut grid = vec![vec![' '; 3]; 3];
        
        // 各手を処理する
        for (i, mv) in moves.iter().enumerate() {
            let row = mv[0] as usize;
            let col = mv[1] as usize;
            // プレイヤーAは'X'、プレイヤーBは'O'を配置
            let symbol = if i % 2 == 0 { 'X' } else { 'O' };
            grid[row][col] = symbol;
        }

        // 行をチェック
        for i in 0..3 {
            if grid[i][0] != ' ' && grid[i][0] == grid[i][1] && grid[i][1] == grid[i][2] {
                return if grid[i][0] == 'X' { "A".to_string() } else { "B".to_string() };
            }
            // 列をチェック
            if grid[0][i] != ' ' && grid[0][i] == grid[1][i] && grid[1][i] == grid[2][i] {
                return if grid[0][i] == 'X' { "A".to_string() } else { "B".to_string() };
            }
        }

        // 斜めをチェック
        if grid[0][0] != ' ' && grid[0][0] == grid[1][1] && grid[1][1] == grid[2][2] {
            return if grid[0][0] == 'X' { "A".to_string() } else { "B".to_string() };
        }
        if grid[0][2] != ' ' && grid[0][2] == grid[1][1] && grid[1][1] == grid[2][0] {
            return if grid[0][2] == 'X' { "A".to_string() } else { "B".to_string() };
        }

        // 全てのマスが埋まっているかをチェック
        if moves.len() == 9 {
            "Draw".to_string() // 引き分け
        } else {
            "Pending".to_string() // まだ手が残っている
        }
    }
}