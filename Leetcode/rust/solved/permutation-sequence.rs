impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut n = n;
        let mut nums = (1..=n as u8).collect::<Vec<_>>();
        let mut n_frac = (1..=n).product::<i32>();
        let mut k = k - 1;
        let mut buf = String::new();
        while n > 0 {
            n_frac /= n;
            let idx = k / n_frac;
            k %= n_frac;
            let num = nums.remove(idx as usize);
            buf.push((num + 48) as char);
            n -= 1;
        }
        buf
    }
}
