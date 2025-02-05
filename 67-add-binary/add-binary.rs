impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut carry = 0;
        let mut result = String::new();

        let mut a_chars = a.chars().rev();
        let mut b_chars = b.chars().rev();

        loop {
            match (a_chars.next(), b_chars.next()) {
                (None, None) => break,
                (a_digit, b_digit) => {
                    let sum = a_digit.unwrap_or('0').to_digit(2).unwrap() + b_digit.unwrap_or('0').to_digit(2).unwrap() + carry;
                    result.push_str(&(sum % 2).to_string());
                    carry = sum / 2;
                }
            }
        }
        if carry > 0 {
            result.push_str(&carry.to_string());
        }
        result.chars().rev().collect()
    }
}