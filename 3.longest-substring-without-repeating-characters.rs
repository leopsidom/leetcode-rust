/*
 * @lc app=leetcode id=3 lang=rust
 *
 * [3] Longest Substring Without Repeating Characters
 */

// @lc code=start
use std::collections::HashMap;
use std::cmp;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map = HashMap::new();
        let chars: Vec<char> = s.chars().collect();
        let mut start = -1;
        let mut ans = 0;

        for i in 0..s.len() {
            if let Some(v) = map.get_mut(&chars[i]) {
                start = cmp::max(start, *v);
            }
            ans = cmp::max(ans, i as i32 - start);
            map.insert(&chars[i], i as i32);
        }

        ans
    }
}
// @lc code=end

