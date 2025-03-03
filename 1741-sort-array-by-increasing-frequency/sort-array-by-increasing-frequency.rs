use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut counts = HashMap::new();

        for &frequency in &nums {
            *counts.entry(frequency).or_insert(0) += 1;
        }
        let mut fq: Vec<(&i32, &i32)> = counts.iter().collect();
        fq.sort_by(|a, b| a.1.cmp(b.1).then_with(||b.0.cmp(a.0)));

        let mut result = Vec::new();
        for (num, &count) in fq {
            for _ in 0..count {
                result.push(*num);
            }
        }
        result
    }
}