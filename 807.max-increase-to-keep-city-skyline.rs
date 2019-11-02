/*
 * @lc app=leetcode id=807 lang=rust
 *
 * [807] Max Increase to Keep City Skyline
 */

// @lc code=start
use std::cmp;

impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let R = grid.len();
        if R == 0 { return 0; }
        let C = grid[0].len();

        let mut top_bottom_view = vec![0; C];
        let mut left_right_view = vec![0; R];

        for i in 0..R {
            for j in 0..C {
                top_bottom_view[j] = cmp::max(top_bottom_view[j], grid[i][j]);
                left_right_view[i] = cmp::max(left_right_view[i], grid[i][j]);
            }
        }

        let mut ans = 0;

        for i in 0..R {
            for j in 0..C {
                ans += cmp::min(top_bottom_view[j], left_right_view[i]) - grid[i][j];
            }
        }

        ans
    }
}
// @lc code=end

