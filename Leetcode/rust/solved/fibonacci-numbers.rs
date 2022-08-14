impl Solution {
    pub fn fib(n: i32) -> i32 {
        (0..n).fold((0, 1), |(s, p), _| (s + p, s)).0
    }
}
