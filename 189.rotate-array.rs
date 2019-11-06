/*
 * @lc app=leetcode id=189 lang=rust
 *
 * [189] Rotate Array
 */

// @lc code=start
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let mut ans = vec![0; nums.len()];

        for i in 0..nums.len() {
            ans[(i+k as usize)%nums.len()] = nums[i];
        }

        ans
    }
}
// @lc code=end

