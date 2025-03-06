impl Solution {
    pub fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut nums1 = nums1.clone();
        let mut nums2 = nums2.clone();
        nums1.sort();
        nums2.sort();

        for &num1 in &nums1 {
            if nums2.contains(&num1) {
                return num1;
            }
        }

        let min1 = nums1[0];
        let min2 = nums2[0];

        (min1 * 10 + min2).min(min2 * 10 + min1)
    }
}