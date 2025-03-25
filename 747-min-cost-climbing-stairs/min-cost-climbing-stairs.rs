impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut c = cost.len();
        if c == 0 {
            return 0
        } else if c == 1 {
            return cost[0]
        }
        let mut up_cost = vec![0; c];

        up_cost[0] = cost[0];
        up_cost[1] = cost[1];

        for i in 2..c {
            up_cost[i] = if up_cost[i - 1] < up_cost[i - 2] { 
                up_cost[i - 1] + cost[i]
                } else { 
                up_cost[i - 2] + cost[i]
                };
        }
        let result = if up_cost[c - 1] < up_cost[c - 2] { up_cost[c - 1] } else { up_cost[c - 2] };
        result
    }
}