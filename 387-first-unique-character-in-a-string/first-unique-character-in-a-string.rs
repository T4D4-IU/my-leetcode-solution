impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        use std::collections::HashMap;
        let mut counts = HashMap::new();

        for c in s.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }
        for (i, c) in s.chars().enumerate() {
            if let Some(&count) = counts.get(&c) {
                if count == 1 {
                    return i as i32;
                }
            }
        }
        -1
    }
}