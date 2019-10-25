/*
 * @lc app=leetcode id=5 lang=rust
 *
 * [5] Longest Palindromic Substring
 */

// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let N = s.len();
        let chars: Vec<char> = s.chars().collect();

        let mut dp = vec![vec![false; N]; N];
        let mut ans = "";
        for j in 0..N {
            for i in (0..j+1).rev() {
                if chars[i] == chars[j] && (j == 0 || i+1 >= j-1 || dp[i+1][j-1]) {
                    dp[i][j] = true;
                    if j - i + 1 > ans.len() {
                        ans = &s[i..j+1];
                    }
                }
            }
        }
        ans.to_string()
    }
}
// @lc code=end

