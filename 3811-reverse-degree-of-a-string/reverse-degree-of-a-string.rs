impl Solution {
    pub fn reverse_degree(s: String) -> i32 {
        s.chars()
            .enumerate()
            .map(|(i, c)| {
                let reserve_pos = reserve_alphabet_position(c);
                let string_pos = (i + 1) as u32;
                (reserve_pos * string_pos) as i32
            })
            .sum()
    }
}

fn reserve_alphabet_position(c: char) -> u32 {
    let base = 'a' as u32;
    let position = c as u32 - base + 1;
    26 - position + 1
}