/*
 * @lc app=leetcode id=171 lang=rust
 *
 * [171] Excel Sheet Column Number
 */

// @lc code=start
impl Solution {
    pub fn title_to_number(s: String) -> i32 {
        let mut ans = 0;

        for c in s.chars() {
            ans *= 26;
            ans += c as i32 - 'A' as i32 + 1;
        }

        ans
    }
}
// @lc code=end

