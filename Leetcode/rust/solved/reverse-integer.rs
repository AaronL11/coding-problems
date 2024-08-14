impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut n = x;
        let s = if n < 0 {
            n = -n;
            -1
        } else {
            1
        };
        let mut ans = 0i32;
        let mut b = false;
        while !b && n > 0 {
            let (q, r) = (n / 10, n % 10);
            let (x, y) = ans.overflowing_mul(10);
            b = y;
            if b {
                return 0;
            }
            ans = x;
            let (x, y) = ans.overflowing_add(r);
            b = y;
            if b {
                return 0;
            }
            ans = x;
            n = q;
        }
        s * ans
    }
}
