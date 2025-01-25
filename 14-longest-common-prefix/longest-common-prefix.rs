impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }
        let mut prefix: Vec<char> = strs[0].chars().collect();

        for s in strs.iter().skip(1) {
            let chars: Vec<char> = s.chars().collect();
            let mut i = 0;

            while i < prefix.len() && i < chars.len() && prefix[i] == chars[i] {
                i += 1
            }

            prefix.truncate(i);

            if prefix.is_empty() {
                break;
            }
        }
        prefix.into_iter().collect()
    }
}