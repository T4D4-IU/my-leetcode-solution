impl Solution {
    pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
        let n = nums.len();

        for i in 0..n {
            for j in 0..n {
                let current_index_diff = if i > j { i - j } else { j - i };

                let current_value_diff = (nums[i] - nums[j]).abs();

                if current_index_diff >= index_difference.try_into().unwrap() && current_value_diff >= value_difference {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![-1, -1]
    }
}