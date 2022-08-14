impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        (0..=n).fold((0, 1), |(s, p), _| (s + p, s)).0
    }
}
