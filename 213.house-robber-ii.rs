/*
 * @lc app=leetcode id=213 lang=rust
 *
 * [213] House Robber II
 */

// @lc code=start
use std::cmp;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {

        let mut rob0_robi = nums[0];
        let mut rob0_norobi = 0;
        let mut norob0_robi = 0;
        let mut norob0_norobi = 0;

        let mut rob = nums[0];
        let mut norob = 0;

        let mut money = (nums[0], 0, 0, 0);

        let N = nums.len();

        for i in 1..N-1 {
            let tmp = rob;
            rob = norob + nums[i];
            norob = cmp::max(rob, norob);
        }
    }
}
// @lc code=end

