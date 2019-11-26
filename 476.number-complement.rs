/*
 * @lc app=leetcode id=476 lang=rust
 *
 * [476] Number Complement
 */

// @lc code=start
impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        if num == 0 { return 1; }
        if num == 1 { return 0; }
        Self::find_complement(num / 2) * 2 + 1 - num % 2
    }
}
// @lc code=end

