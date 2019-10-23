/*
 * @lc app=leetcode id=28 lang=rust
 *
 * [28] Implement strStr()
 */

// @lc code=start
use std::collections::LinkedList;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let m = haystack.len();
        let n = needle.len();

        let mut ans = -1;

        if (m < n) {
            return ans;
        }


        for i in 0..(m-n+1) {
            let x = haystack.get(i..(i+n));
            // println!("{}", x.unwrap());
            match x {
                Some(s) => {
                    if (s == needle) {
                        ans = i as i32;
                        break;
                    }
                },
                None => continue
            }
        }

        return ans;
    }
}
// @lc code=end

