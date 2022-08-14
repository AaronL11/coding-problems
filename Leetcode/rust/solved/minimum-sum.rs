use std::cmp::min;

const INF: i32 = 1_000_000;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = [[INF; 202]; 202];
        let (m, n) = (grid[0].len(), grid.len());
        dp[0][1] = 0;
        dp[1][0] = 0;
        for r in 1..=n {
            for c in 1..=m {
                dp[r][c] = grid[r - 1][c - 1] + min(dp[r - 1][c], dp[r][c - 1]);
            }
        }
        dp[n][m]
    }
}
