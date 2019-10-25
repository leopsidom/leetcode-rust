/*
 * @lc app=leetcode id=231 lang=rust
 *
 * [231] Power of Two
 */

// @lc code=start
impl Solution {
    pub fn is_power_of_two(mut n: i32) -> bool {
        if n == 0 {
            return false;
        }
        
        while n & 1 == 0 {
            n >>= 1;
        }

        n == 1
    }
}
// @lc code=end

