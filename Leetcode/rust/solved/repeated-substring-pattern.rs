impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let v = s.as_bytes();
        (1..v.len())
            .filter(|n| v.len() % n == 0)
            .any(|n| v.chunks(n).all(|w| w == &v[..n]))
    }
}