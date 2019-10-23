/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let size = nums.len();
        let mut indices = HashMap::new();

        for i in 0..size {
            let val = nums.get(i).unwrap();
            if (indices.contains_key(&(target - val))) {
                let j = indices.get(&(target - val)).unwrap();
                return vec![*j as i32, i as i32];
            }
            indices.insert(val, i);
        }

        return vec![0, 0];
    }
}
// @lc code=end

