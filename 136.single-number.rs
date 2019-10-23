/*
 * @lc app=leetcode id=136 lang=rust
 *
 * [136] Single Number
 */

// @lc code=start
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut ans = 0;

        for num in nums {
            ans ^= num;
        }

        ans
    }
}
// @lc code=end

