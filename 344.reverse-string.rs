/*
 * @lc app=leetcode id=344 lang=rust
 *
 * [344] Reverse String
 */

// @lc code=start
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let N = s.len();
        for i in 0..N / 2 { s.swap(i, N-i-1); }
    }
}
// @lc code=end

