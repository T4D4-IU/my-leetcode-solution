impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut result = num;
        let mut count = 0;
        while result !=  0 {
            if result % 2 == 0 {
                // even
                result = result / 2;
            } else {
                // odd
                result = result - 1;
            }
            count += 1;
        }
        count
    }
}