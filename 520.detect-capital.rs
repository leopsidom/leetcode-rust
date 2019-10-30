/*
 * @lc app=leetcode id=520 lang=rust
 *
 * [520] Detect Capital
 */

// @lc code=start
impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut is_uppercase = true;

        let chars: Vec<char> = word.chars().collect();
        if chars.len() <= 1 { return true; }

        for i in 1..chars.len() {
            if chars[i].is_ascii_lowercase() {
                if i == 1 {
                    is_uppercase = false;
                } else {
                    if is_uppercase {
                        return false;
                    }
                }
            } else if !is_uppercase {
                return false;
            }
        }

        return chars[0].is_ascii_uppercase() || (chars[0].is_ascii_lowercase() && !is_uppercase);
    }
}
// @lc code=end

