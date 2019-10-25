/*
 * @lc app=leetcode id=169 lang=rust
 *
 * [169] Majority Element
 */

// @lc code=start
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut cnt = 1;
        let mut val = nums[0];

        for i in 1..nums.len() {
            if val != nums[i] {
                if cnt == 0 {
                    val = nums[i];
                } else {
                    cnt -= 1;
                }
            } else {
                cnt += 1;
            }


        }

        val
    }
}
// @lc code=end

