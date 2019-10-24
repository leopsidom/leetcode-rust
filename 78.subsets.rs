/*
 * @lc app=leetcode id=78 lang=rust
 *
 * [78] Subsets
 */

// @lc code=start
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let size = (2 as usize).pow(nums.len() as u32);
        let mut ans = vec![];

        for i in 0..size {
            let mut s = vec![];
            for j in 0..nums.len() {
                if i & (1 << j) != 0 {
                    s.push(nums[j]);
                }
            }
            ans.push(s);
        }

        ans
    }
}
// @lc code=end

