impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n <= 0 {
            false
        } else {
            let log = (n as f64).log(3.).round() as u32;
            n == 3i32.pow(log)
        }
    }
}
