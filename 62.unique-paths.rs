/*
 * @lc app=leetcode id=62 lang=rust
 *
 * [62] Unique Paths
 */

// @lc code=start
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m <= 0 || n <= 0 {
            return 0;
        }

        let mut dp = vec![vec![1; n as usize]; m as usize];

        for i in (0..m as usize).rev() {
            for j in (0..n as usize).rev() {
                if i == (m - 1) as usize {
                    if j < (n - 1) as usize {
                        dp[i][j] = dp[i][j+1];
                    }
                } else {
                    if j == (n - 1) as usize {
                        dp[i][j] = dp[i+1][j];
                    } else {
                        dp[i][j] = dp[i+1][j] + dp[i][j+1];
                    }
                }
            }
        }

        dp[0][0]
    }
}
// @lc code=end

