use std::collections::HashMap;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut counts = HashMap::new();

        // 文字の出現回数をカウント
        for c in s.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }

        // 最初の一度だけ登場する文字のインデックスを探す
        for (i, c) in s.chars().enumerate() {
            if counts[&c] == 1 {
                return i as i32;
            }
        }

        -1
    }
}