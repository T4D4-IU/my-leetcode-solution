impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let n = haystack.len();
        let m = needle.len();
        
        // もしneedleが空文字列であれば、0を返す
        if m == 0 {
            return 0;
        }

        if n < m {
            return -1;
        }

        // haystackの中でneedleの最初の出現位置を見つける
        for i in 0..=n - m {
            // haystackの部分文字列とneedleを比較
            if &haystack[i..i + m] == needle {
                return i as i32;
            }
        }
        
        // 見つからなければ-1を返す
        -1
    }
}