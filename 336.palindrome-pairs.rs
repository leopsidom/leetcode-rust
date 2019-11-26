/*
 * @lc app=leetcode id=336 lang=rust
 *
 * [336] Palindrome Pairs
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        words.sort_by_key(|s| s.len());

        let mut prefix = HashMap::new();
        let mut suffix = HashMap::new();

        for i in 0..words.len() {

            for 
            if suffix.contains_key(k: &Q)

            let mut pindices = prefix.get_mut(&words[i]);
            if let Some(pind) = pindices {
                (*pind).push(i);
            } else {
                *pind = vec![];
            }

            let mut sindices = suffix.get_mut(&words[i].chars().rev().collect()::<String>);
            if let Some(sind) = sindices {
                (*sind).push(i);
            } else {
                *pind = vec![i];
            }


        }
    }
}
// @lc code=end

