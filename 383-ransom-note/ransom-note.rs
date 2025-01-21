use std::collections::HashMap;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }

        let ransom_count = count_characters(&ransom_note);
        let magazine_count = count_characters(&magazine);

        for (char, &count) in ransom_count.iter() {
            if count > *magazine_count.get(&char).unwrap_or(&0) {
                return false;
            }
        }
        true

    }
}

fn count_characters(s: &str) -> HashMap<char, i32> {
    let mut char_count = HashMap::new();
    for c in s.chars() {
        *char_count.entry(c).or_insert(0) += 1;
    }
    char_count
}