use std::collections::HashMap;
impl Solution {
    pub fn most_visited(n: i32, rounds: Vec<i32>) -> Vec<i32> {
        let mut sector_count = vec![0; (n + 1) as usize];
        let m = rounds.len();

        for i in 0..(m - 1) {
            let start = rounds[i];
            let end = rounds[i + 1];
            let mut sector = start;
            while sector != end {
                sector_count[sector as usize] += 1;
                sector = if sector == n { 1 } else { sector + 1};
            }
        }
        sector_count[*rounds.last().unwrap() as usize] += 1;

        let max_visits = *sector_count.iter().max().unwrap();
        let mut result = vec![];

        for i in 1..=n {
            if sector_count[i as usize] == max_visits {
                result.push(i);
            }
        }
        result
    }
}