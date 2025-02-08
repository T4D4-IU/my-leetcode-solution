impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }

        let mut one_step_before = 2;
        let mut two_steps_before = 1;
        let mut all_ways = 0;

        for _ in 3..=n {
            all_ways = one_step_before + two_steps_before;
            two_steps_before = one_step_before;
            one_step_before = all_ways;
        }
        all_ways
    }
}