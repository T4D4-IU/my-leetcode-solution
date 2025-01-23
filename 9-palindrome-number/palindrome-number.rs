impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let x_str = x.to_string();

        let x_len = x_str.len();

        let x_chars: Vec<char> = x_str.chars().collect();

        for i in 0..x_len / 2 {
            if x_chars[i] != x_chars[x_len - 1 -i] {
                return false;
            }
        } 
        true
    }
}