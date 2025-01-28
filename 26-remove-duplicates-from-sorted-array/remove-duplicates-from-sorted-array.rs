impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut idx = 0;

        for i in 1..nums.len() {
            if nums[i] != nums[idx] {
                idx += 1;
                nums[idx] = nums[i];
            }
        }
        (idx + 1) as i32
    }
}