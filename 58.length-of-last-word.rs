/*
 * @lc app=leetcode id=58 lang=rust
 *
 * [58] Length of Last Word
 */

// @lc code=start
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {

        let n = s.len() as i32;
        let mut last = -1;
        let mut ans = 0;

        for (i, c) in s.chars().enumerate() {
            if c == ' ' {
                if i as i32 - last - 1 != 0 {
                    ans = i as i32 - last - 1;
                }
                last = i as i32;
            }
        }

        if (n - last - 1 != 0) {
            ans = n - last - 1;
        }
        ans
    }
}
// @lc code=end

