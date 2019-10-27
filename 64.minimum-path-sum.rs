/*
 * @lc app=leetcode id=64 lang=rust
 *
 * [64] Minimum Path Sum
 */

// @lc code=start
use std::cmp;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let R = grid.len();
        if R == 0 { return 0; }

        let C = grid[0].len();
        if C == 0 { return 0; }


        let mut dp = vec![vec![1; C as usize]; R as usize];

        for i in (0..R as usize).rev() {
            for j in (0..C as usize).rev() {
                if i == (R - 1) as usize {
                    if j < (C - 1) as usize {
                        dp[i][j] = dp[i][j+1] + grid[i][j];
                    } else {
                        dp[i][j] = grid[i][j];
                    };
                } else {
                    if j == (C - 1) as usize {
                        dp[i][j] = dp[i+1][j] + grid[i][j];
                    } else {
                        dp[i][j] = cmp::min(dp[i+1][j], dp[i][j+1]) + grid[i][j];
                    }
                }
            }
        }

        dp[0][0]
    }
}
// @lc code=end

