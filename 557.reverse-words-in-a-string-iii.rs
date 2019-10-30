/*
 * @lc app=leetcode id=557 lang=rust
 *
 * [557] Reverse Words in a String III
 */

// @lc code=start
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let iter = s.split(' ');

        let mut ans = String::new();
        for word in iter {
            if ans.len() != 0 {
                ans.push(' ');
            }
            let revWord: String = word.chars().rev().collect();
            ans.push_str(&revWord);
        }
        ans
    }
}
// @lc code=end

