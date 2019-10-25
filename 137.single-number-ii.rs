/*
 * @lc app=leetcode id=137 lang=rust
 *
 * [137] Single Number II
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        Self::bit_manipulation_sol(nums)
    }

    pub fn hashmap_sol(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();

        for num in nums {
            if let Some(v) = map.get_mut(&num) {
                *v += 1;
            } else {
                map.insert(num, 1);
            }
        }

        for k in &map {
            if *k.1 == 1 {
                return *k.0;
            }
        }
        
        0
    }

    pub fn bit_manipulation_sol(nums: Vec<i32>) -> i32 {
        let mut x1 = 0;
        let mut x2 = 0;
        let mut mask = 0;

        for num in nums {
            x2 ^= x1 & num;
            x1 ^= num;
            mask = !(x1 & x2);
            x2 &= mask;
            x1 &= mask;
        }

        x1
    }
}
// @lc code=end

