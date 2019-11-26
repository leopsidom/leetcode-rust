/*
 * @lc app=leetcode id=290 lang=rust
 *
 * [290] Word Pattern
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn word_pattern(pattern: String, str: String) -> bool {
        let mut forward = HashMap::new();
        let mut backward = HashMap::new();

        let words: Vec<&str> = str.split(' ').collect();
        if pattern.len() != words.len() {
            return false;
        }
        for (i, ch) in pattern.chars().enumerate() {
            if forward.contains_key(&ch) {
                if forward.get(&ch).unwrap() != words[i] {
                    return false;
                }
            } else {
                forward.insert(ch, words[i].to_owned());
            }

            if backward.contains_key(words[i]) {
                if backward.get(words[i]).unwrap() != &ch {
                    return false;
                }
            } else {
                backward.insert(words[i].to_owned(), ch);
            }
        }

        return true;
    }
}
// @lc code=end

