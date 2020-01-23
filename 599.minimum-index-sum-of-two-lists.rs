/*
 * @lc app=leetcode id=599 lang=rust
 *
 * [599] Minimum Index Sum of Two Lists
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut str_2_idx = HashMap::new();

        for (i, s) in list1.into_iter().enumerate() {
            if !str_2_idx.contains_key(&s) {
                str_2_idx.insert(s, i);
            }
        }

        let mut ans = HashMap::new();

        for (i, s) in list2.into_iter().enumerate() {
            match str_2_idx.get(&s) {
                Some(j) => {
                    ans.entry(i + *j).or_insert(vec![]).push(s);
                },
                None => ()
            }
        }

        ans.get(ans.keys().min().unwrap()).unwrap().to_vec()
    }
}
// @lc code=end

