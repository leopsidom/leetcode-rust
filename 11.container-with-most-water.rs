/*
 * @lc app=leetcode id=11 lang=rust
 *
 * [11] Container With Most Water
 */

// @lc code=start
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut ans = 0;

        while left < right {
            if height[left] < height[right] {
                let area = (right - left) as i32 * height[left];
                if (ans < area) {
                    ans = area;
                }
                left += 1;
            }
            else {
                let area = (right - left) as i32 * height[right];
                if (ans < area) {
                    ans = area;
                }
                right -= 1;
            }
        }

        ans
    }
}
// @lc code=end

