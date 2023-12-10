fn trailing_zeroes(n: i64) -> i64 {
    if n == 0 {
        0
    } else {
        n / 5 + trailing_zeroes(n / 5)
    }
}

impl Solution {
    pub fn preimage_size_fzf(k: i32) -> i32 {
        let k = k as i64;
        let mut max = 5 * k + 1 as i64;
        let mut min = 0;
        while min < max {
            let mid = (max + min) / 2;
            let trail = trailing_zeroes(mid);
            if k < trail {
                max = mid;
            } else if k > trail {
                min = mid + 1;
            } else {
                return 5;
            }
        }
        0
    }
}
