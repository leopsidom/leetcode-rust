/*
 * @lc app=leetcode id=7 lang=rust
 *
 * [7] Reverse Integer
 */

// @lc code=start
use std::i32;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let sign = x.signum();
        let mut val = x.abs();
        let mut ans = 0 as i64;

        while (val > 0) {
            let last = (val % 10) as i64;
            val /= 10;
            ans = ans * 10 + last;
        }

        return if (ans > i32::MAX as i64) { 0 } else { sign * ans as i32 };
    }
}
// @lc code=end

