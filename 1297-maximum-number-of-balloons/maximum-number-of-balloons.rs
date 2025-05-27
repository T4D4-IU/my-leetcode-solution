impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        use std::collections::HashMap;
        let mut char_counts: HashMap<char, i32> = HashMap::new();
        
        // "balloon"を構成する文字列のみをカウント対象とする
        let target_chars = ['b', 'a', 'l', 'o', 'n'];

        // HashMapに初期値を入れる
        char_counts.insert('b', 0);
        char_counts.insert('a', 0);
        char_counts.insert('l', 0);
        char_counts.insert('o', 0);
        char_counts.insert('n', 0);

        for c in text.chars() {
            if target_chars.contains(&c) {
                *char_counts.entry(c).or_insert(0) += 1;
            }
        }

        // lと0は２回使うので半分にする。
        char_counts.entry('l')
            .and_modify(|count| {
                *count /= 2
            });
        char_counts.entry('o')
            .and_modify(|count| {
                *count /= 2
            });
        

        // 一番数が少ない文字の数だけballonを作れる
        if let Some(min) = char_counts.values().min() {
            *min
        } else {
            0
        }
    }
}