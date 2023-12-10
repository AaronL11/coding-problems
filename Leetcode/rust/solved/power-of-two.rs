impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n <= 0 {
            false
        } else {
            (n as u32).is_power_of_two()
        }
    }
}
