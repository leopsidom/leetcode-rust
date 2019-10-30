/*
 * @lc app=leetcode id=345 lang=rust
 *
 * [345] Reverse Vowels of a String
 */

// @lc code=start
use std::collections::HashSet;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {

        let mut chars: Vec<char> = s.chars().collect();
        if chars.len() <= 1 { return s; }
        let vowels: HashSet<char> = 
            ['a', 'e', 'i', 'o', 'u',
             'A', 'E', 'I', 'O', 'U'].iter().cloned().collect();

        let mut lo = 0;
        let mut hi = chars.len() - 1;

        while lo < hi {
            while lo < hi && !vowels.contains(&chars[lo]) { lo += 1; }
            while lo < hi && !vowels.contains(&chars[hi]) { hi -= 1; }
            if lo < hi {
                chars.swap(lo, hi);
                lo += 1;
                hi -= 1;
            }
        }

        chars.into_iter().collect()
    }
}
// @lc code=end

