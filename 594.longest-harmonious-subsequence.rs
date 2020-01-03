/*
 * @lc app=leetcode id=594 lang=rust
 *
 * [594] Longest Harmonious Subsequence
 */

// @lc code=start
use std::collections::HashMap;
use std::cmp;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut counter = HashMap::new();

        for num in nums {
            *counter.entry(num).or_insert(0) += 1;
        }

        let mut ans = 0;
        for key in counter.keys() {
            if counter.contains_key(&(key + 1)) {
                ans = cmp::max(ans, 
                    counter.get(&key).unwrap() + 
                    counter.get(&(key+1)).unwrap()
                );
            }
        }

        ans
    }
}
// @lc code=end

