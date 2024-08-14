fn f(n: i32) -> i32 {
    if n == 0 {
        0
    } else {
        n / 5 + f(n / 5)
    }
}

impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        f(n)
    }
}
