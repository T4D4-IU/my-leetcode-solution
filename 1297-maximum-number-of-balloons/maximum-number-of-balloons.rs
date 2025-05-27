use std::collections::HashMap;

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        // 1. text 内の各文字の出現回数をカウントする
        let mut text_counts = HashMap::new();
        for char_c in text.chars() {
            *text_counts.entry(char_c).or_insert(0) += 1;
        }

        // 2. "balloon" を作るのに必要な文字の出現回数をカウントする
        //    実際には固定なので、直接値を使います。
        //    b: 1, a: 1, l: 2, o: 2, n: 1

        // 3. 各文字で作れる "balloon" の数を計算する
        let count_b = *text_counts.get(&'b').unwrap_or(&0);
        let count_a = *text_counts.get(&'a').unwrap_or(&0);
        let count_l = *text_counts.get(&'l').unwrap_or(&0) / 2; // 'l' は2つ必要
        let count_o = *text_counts.get(&'o').unwrap_or(&0) / 2; // 'o' は2つ必要
        let count_n = *text_counts.get(&'n').unwrap_or(&0);

        // 4. これらのうち最小のものが作れる "balloon" の最大数
        let mut result = count_b;
        result = result.min(count_a);
        result = result.min(count_l);
        result = result.min(count_o);
        result = result.min(count_n);

        result
    }
}