impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            false
        } else {
            let s = x.to_string().into_bytes();
            let n = s.len();
            (0..n / 2).all(|i| s[i] == s[n - 1 - i])
        }
    }
}

