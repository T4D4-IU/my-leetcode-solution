impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut counts = HashMap::new();

        for i in nums {
            *counts.entry(i).or_insert(0) += 1;
        }
        
        let mut result = Vec::new();
        for (key, value) in counts {
            if value == 2 {
                result.push(key);
            }
        }
        result
    }
}