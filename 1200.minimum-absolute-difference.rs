/*
 * @lc app=leetcode id=1200 lang=rust
 *
 * [1200] Minimum Absolute Difference
 */

// @lc code=start
use std::collections::HashMap;
use std::i32;

impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut diff_map = HashMap::new();
        let mut min_diff = i32::MAX;
        arr.sort();

        for i in 0..arr.len()-1 {
            let d = arr[i+1] - arr[i];
            if d < min_diff { min_diff = d; }
            diff_map.entry(d).or_insert(vec![]).push(vec![arr[i], arr[i+1]]);
        }

        match diff_map.get(&min_diff) {
            Some(v) => v.clone(),
            None => vec![]
        }
    }
}
// @lc code=end

