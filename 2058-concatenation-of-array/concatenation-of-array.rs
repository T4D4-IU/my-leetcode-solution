impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = nums.clone();
        result.extend(nums);
        result
    }
}