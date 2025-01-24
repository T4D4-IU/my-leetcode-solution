use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let map = {
            let mut map=HashMap::new();
            map.insert('I', 1);
            map.insert('V', 5);
            map.insert('X', 10);
            map.insert('L', 50);
            map.insert('C', 100);
            map.insert('D', 500);
            map.insert('M', 1000);
            map
        };

        let mut result = 0;
        let chars: Vec<char> = s.chars().collect();
        
        for i in 0..chars.len() {
            let current_value = *map.get(&chars[i]).unwrap();
            if i < chars.len() - 1 {
                let next_value = *map.get(&chars[i + 1]).unwrap();
                if current_value < next_value {
                    result -= current_value;
                } else {
                    result += current_value;
                }
            } else {
                result += current_value;
            }
        }
        result
    }
}