use std::collections::HashSet;
impl Solution {
    pub fn count_vowel_substrings(word: String) -> i32 {
        let vowels: HashSet<char> = ['a', 'i', 'u', 'e', 'o'].iter().cloned().collect();
        let mut result = 0;

        for i in 0..word.len() {
            let mut vowel_set = HashSet::new();
            for j in i..word.len() {
                let c = word.chars().nth(j).unwrap();
                if vowels.contains(&c) {
                    vowel_set.insert(c);
                    if vowel_set.len() == 5 {
                        result += 1;
                    }
                } else {
                    break;
                }
            }
        }
        result
    }
}