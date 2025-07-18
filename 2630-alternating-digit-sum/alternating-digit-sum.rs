impl Solution {
    pub fn alternate_digit_sum(n: i32) -> i32 {
        // nを文字列に変換
        let n_str = n.to_string();

        let char_iterator = n_str.chars();

        let sum = char_iterator
            .enumerate()
            .map(|(i, c)| {
                let digit = c.to_digit(10).unwrap() as i32;
                if i % 2 == 0 {
                    digit
                } else {
                    -digit
                }
            })
            .sum();
        sum
    }
}