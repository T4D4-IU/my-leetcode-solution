impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let x_str = x.to_string();

        x_str == x_str.chars().rev().collect::<String>()
    }
}