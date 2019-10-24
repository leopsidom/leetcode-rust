/*
 * @lc app=leetcode id=55 lang=rust
 *
 * [55] Jump Game
 */

// @lc code=start
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        Self::fast_jump(nums)
    }

    pub fn fast_jump(nums: Vec<i32>) -> bool {
        let N = nums.len();
        let mut last = N-1;

        for i in (0..N-1).rev() {
            if (i + nums[i] as usize >= last) {
                last = i;
            }
        }

        last <= 0
    }

    pub fn naive_jump(nums: Vec<i32>) -> bool {
        let N = nums.len();
        if N <= 1 {
            return true;
        }

        let mut dp = vec![false; N];
        dp[N-1] = true;

        for i in (0..N-1).rev() {
            for s in 0..nums[i] {
                if dp[i+s as usize+1] {
                    dp[i] = true;
                    break;
                }
            }
        }

        dp[0]
    }
}
// @lc code=end

