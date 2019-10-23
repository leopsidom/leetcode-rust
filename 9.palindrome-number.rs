/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if (x == 0) {
            return true;
        }
        if (x < 0 || x % 10 == 0) {
            return false;
        }

        let mut reverse = 0;
        let mut part = x;
        
        while (part > 0) {
            reverse = reverse * 10 + part % 10;
            part /= 10;
        }

        return x == reverse;
    }
}
// @lc code=end

