/*
 * @lc app=leetcode id=647 lang=rust
 *
 * [647] Palindromic Substrings
 */

// @lc code=start
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let chars = s.chars().collect::<Vec<char>>();
        let N = chars.len();

        let mut ans = 0;
        let mut dp = vec![vec![false; N]; N];

        for j in 0..N {
            for i in (0..j+1).rev() {
                if chars[i] == chars[j] && (i as i32 + 1 >= j as i32 - 1 || dp[i+1][j-1]) {
                    ans += 1;
                    dp[i][j] = true;
                }
            }
        }
        
        ans
    }
}
// @lc code=end

