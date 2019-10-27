/*
 * @lc app=leetcode id=17 lang=rust
 *
 * [17] Letter Combinations of a Phone Number
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.len() == 0 {
            return vec![];
        }
        let mut ans = vec!["".to_string()];
        let map: HashMap<char, Vec<char>> = [
            ('2', vec!['a', 'b', 'c']),
            ('3', vec!['d', 'e', 'f']),
            ('4', vec!['g', 'h', 'i']),
            ('5', vec!['j', 'k', 'l']),
            ('6', vec!['m', 'n', 'o']),
            ('7', vec!['p', 'q', 'r', 's']),
            ('8', vec!['t', 'u', 'v']),
            ('9', vec!['w', 'x', 'y', 'z'])
        ].iter().cloned().collect();

        for d in digits.chars() {
            let mut tmp = vec![];
            if let Some(v) = map.get(&d) {
                for s in &ans {
                    for ch in v {
                        tmp.push(s.clone() + &ch.to_string());
                    }
                }
            }
            ans = tmp;
        }

        ans
    }
}
// @lc code=end

