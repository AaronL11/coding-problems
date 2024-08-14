fn happy(n: i32) -> i32 {
    let mut n = n;
    let mut sum = 0;
    while n > 0 {
        let x = n % 10;
        n /= 10;
        sum += x * x;
    }
    sum
}

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut n = happy(n);
        let mut m = happy(happy(n));
        loop {
            if n == 1 {
                return true;
            } else if n == m {
                return false;
            }
            n = happy(n);
            m = happy(happy(m));
        }
    }
}
