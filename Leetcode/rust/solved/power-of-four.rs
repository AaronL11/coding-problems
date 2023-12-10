impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        if n == 1 {
            true
        } else if n <= 0 || n % 2 == 1 {
            false
        } else {
            n == 4i32.pow(dbg!((n as f64).log(4.).round() as u32))
        }
    }
}
