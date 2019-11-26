/*
 * @lc app=leetcode id=45 lang=rust
 *
 * [45] Jump Game II
 */

// @lc code=start
use std::i32;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        // Self::jump_dp(nums)
        Self::jump_bsf(nums)
    }

    pub fn jump_dp(nums: Vec<i32>) -> i32 {
        let N = nums.len();
        if N <= 1 { return 0; }

        let mut dp = vec![i32::MAX; N];
        dp[N-1] = 0;

        for i in (0..N-1).rev() {
            let mut minstep = dp[i+1];
            for k in (1..nums[i] as usize + 1) {
                if i + k >= N {
                    break;
                }
                if dp[i+k] < minstep {
                    minstep = dp[i+k];
                }
            }
            dp[i] = minstep + 1;
        }

        dp[0]
    }

    pub fn jump_bsf(nums: Vec<i32>) -> i32 {
        let mut cur_end = 0;
        let mut cur_furthest = 0;
        let mut jumps = 0;

        let N = nums.len();

        for i in 0..N-1 {
            if i + nums[i] as usize > cur_furthest {
                cur_furthest = i + nums[i] as usize;
            }
            if i == cur_end {
                jumps += 1;
                cur_end = cur_furthest;
            }
        }

        jumps
    }
}
// @lc code=end

