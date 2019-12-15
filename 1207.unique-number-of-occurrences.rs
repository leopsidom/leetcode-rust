/*
 * @lc app=leetcode id=1207 lang=rust
 *
 * [1207] Unique Number of Occurrences
 */

// @lc code=start
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut counter = HashMap::new();

        for ele in arr {
            *counter.entry(ele).or_insert(0) += 1;
        }

        counter.values().collect::<HashSet<_>>().len() == counter.len()
    }
}
// @lc code=end

