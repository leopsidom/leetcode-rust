/*
 * @lc app=leetcode id=696 lang=rust
 *
 * [696] Count Binary Substrings
 */

// @lc code=start
impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let chars = s.chars().collect::<Vec<char>>();
        let N = chars.len();

        if N <= 1 { return 0; }
        let mut ans = 0;

        for i in 0..N-1 {
            if (chars[i] == '0' && chars[i+1] == '1') ||
               (chars[i] == '1' && chars[i+1] == '0') {
                   let mut k = 1;
                   while i + k < N && i as i32 - k as i32 + 1 >= 0 {
                       if chars[i+k] == chars[i+1] && chars[i-k+1] == chars[i] {
                           k += 1;
                           ans += 1;
                       } else {
                           break;
                       }
                   }
               }
        }

        ans
    }
}
// @lc code=end

