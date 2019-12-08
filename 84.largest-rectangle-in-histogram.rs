/*
 * @lc app=leetcode id=84 lang=rust
 *
 * [84] Largest Rectangle in Histogram
 */

// @lc code=start
use std::cmp;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let N = heights.len();

        let mut less_from_left = vec![0; N];
        let mut less_from_right = vec![0; N];

        for i in 0..N {
            let mut p = i as i32 - 1;

            while p >= 0 && heights[p as usize] >= heights[i] {
                p = less_from_left[p as usize];
            }

            less_from_left[i] = p;
        }

        for i in (0..N).rev() {
            let mut p = i as i32 + 1;

            while p < N as i32 && heights[p as usize] >= heights[i] {
                p = less_from_right[p as usize];
            }

            less_from_right[i] = p;
        }

        let mut ans = 0;
        for i in 0..N {
            ans = cmp::max(ans, heights[i] * (less_from_right[i] - less_from_left[i] - 1));
        }

        ans
    }
}
// @lc code=end

