use std::cmp::max;

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let b = s.into_bytes();
        let n = b.len();
        let mut lps = [[0; 1002]; 1002];
        for i in (1..=n).rev() {
            for j in i..=n {
                lps[i][j] = if i == j {
                    1
                } else if b[i - 1] == b[j - 1] {
                    if i + 1 == j {
                        2
                    } else {
                        2 + lps[i + 1][j - 1]
                    }
                } else {
                    max(lps[i + 1][j], lps[i][j - 1])
                }
            }
        }
        lps[1][n]
    }
}
