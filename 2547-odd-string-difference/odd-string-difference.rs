use std::collections::HashMap;

impl Solution {
    pub fn odd_string(words: Vec<String>) -> String {
        let mut difference_map: HashMap<Vec<i32>, Vec<String>> = HashMap::new();
        for word in &words {
            let difference = calculate_difference(word);
            difference_map.entry(difference).or_insert(Vec::new()).push(word.clone());
        }

        let mut major_difference: Option<&Vec<i32>> = None;
        let mut max_count = 0;
        for (difference, word_list) in &difference_map {
            if word_list.len() > max_count {
                max_count = word_list.len();
                major_difference = Some(difference);
            }
        }

        for (difference, word_list) in &difference_map {
            if Some(difference) != major_difference {
                return word_list[0].clone();
            }
        }
        words[0].clone()
    }
}

fn calculate_difference(word: &str)-> Vec<i32> {
    let chars: Vec<char> = word.chars().collect();
       let mut differences = Vec::new();
       for i in 0..chars.len() - 1 {
        differences.push((chars[i + 1] as i32) - (chars[i] as i32));
       }
       differences
}

