impl Solution {
    pub fn check_string(s: String) -> bool {
        let mut p_a = 0;
        let mut p_b = 0;
        let mut seen_b = false;

        for (i, c) in s.chars().enumerate() {
            if c == 'a' {
                p_a = i;
            } else if seen_b == false {
                p_b = i;
                seen_b = true;
            }
        }
        if p_a <= p_b {
            true
        } else if seen_b == false {
            true
        } else {
            false
        }
    }
}