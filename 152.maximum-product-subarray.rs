/*
 * @lc app=leetcode id=152 lang=rust
 *
 * [152] Maximum Product Subarray
 */

// @lc code=start
use std::cmp;
use std::mem;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut imax = nums[0];
        let mut imin = nums[0];
        
        let mut ans = nums[0];

        for i in 1..nums.len() {
            if nums[i] < 0 {
                mem::swap(&mut imax, &mut imin);
            }
            imax = cmp::max(nums[i], imax * nums[i]);
            imin = cmp::min(nums[i], imin * nums[i]);

            ans = cmp::max(ans, imax);
        }

        ans
    }
}
// @lc code=end

