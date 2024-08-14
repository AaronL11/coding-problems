impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let n = needle.len();
        haystack
            .as_bytes()
            .windows(n)
            .flat_map(std::str::from_utf8)
            .enumerate()
            .filter_map(|(i, x)| if *x == needle { Some(i as i32) } else { None })
            .next()
            .unwrap_or(-1)
    }
}
