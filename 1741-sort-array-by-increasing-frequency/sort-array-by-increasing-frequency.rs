use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut counts = HashMap::new();

        // Count the frequency of each element in nums
        for &num in &nums {
            *counts.entry(num).or_insert(0) += 1;
        }

        // Create a vector of nums and sort it based on the frequency and value
        let mut sorted_nums = nums.clone();
        sorted_nums.sort_by(|a, b| {
            let count_a = counts.get(a).unwrap();
            let count_b = counts.get(b).unwrap();
            count_a.cmp(count_b).then_with(|| b.cmp(a))
        });

        sorted_nums
    }
}