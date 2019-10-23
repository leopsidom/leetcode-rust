/*
 * @lc app=leetcode id=53 lang=rust
 *
 * [53] Maximum Subarray
 */

// @lc code=start
use std::i32;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut ans = i32::MIN;
        let mut sum = 0;

        for &num in nums.iter() {
            if (sum < 0) {
                sum = num;
            } else {
                sum += num;
            }

            if sum > ans {
                ans = sum
            }
        }

        return ans;
    }
}
// @lc code=end

