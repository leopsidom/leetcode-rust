/*
 * @lc app=leetcode id=63 lang=rust
 *
 * [63] Unique Paths II
 */

// @lc code=start
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let R = obstacle_grid.len();
        if R == 0 { return 0; }

        let C = obstacle_grid[0].len();
        if C == 0 { return 0; }


        let mut dp = vec![vec![1; C as usize]; R as usize];

        for i in (0..R as usize).rev() {
            for j in (0..C as usize).rev() {
                if i == (R - 1) as usize {
                    if j < (C - 1) as usize {
                        if obstacle_grid[i][j] == 1 {
                            dp[i][j] = 0;
                        } else {
                            dp[i][j] = dp[i][j+1];
                        }
                    } else {
                        if obstacle_grid[i][j] == 1 {
                            dp[i][j] = 0;
                        } else {
                            dp[i][j] = 1;
                        }
                    };
                } else {
                    if j == (C - 1) as usize {
                        if obstacle_grid[i][j] == 1 {
                            dp[i][j] = 0;
                        } else {
                            dp[i][j] = dp[i+1][j];
                        }
                    } else {
                        if obstacle_grid[i][j] == 1 {
                            dp[i][j] = 0;
                        } else {
                            dp[i][j] = dp[i+1][j] + dp[i][j+1];
                        }
                    }
                }
            }
        }

        dp[0][0]
    }
}
// @lc code=end

