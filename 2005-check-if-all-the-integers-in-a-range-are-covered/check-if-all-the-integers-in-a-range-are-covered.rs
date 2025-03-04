impl Solution {
    pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
        for i in left..=right { 
            let mut coverd = false;
            for range in &ranges {
                if range[0] <= i && i <= range[1] {
                    coverd = true;
                    break;
                }
            }
            if !coverd {
                return false;
            }
        }
        true
    }
}