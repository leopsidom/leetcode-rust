/*
 * @lc app=leetcode id=583 lang=rust
 *
 * [583] Delete Operation for Two Strings
 */

// @lc code=start
use std::cmp;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let chars1 = word1.chars().collect::<Vec<char>>();
        let chars2 = word2.chars().collect::<Vec<char>>();

        let l1 = chars1.len();
        let l2 = chars2.len();

        let mut dp = vec![vec![0; l2+1]; l1+1];

        for i in 0..l1+1 {
            for j in 0..l2+1 {
                if i == 0 || j == 0 {
                    dp[i][j] = 0;
                } else if chars1[i-1] == chars2[j-1] {
                    dp[i][j] = 1 + dp[i-1][j-1];
                } else {
                    dp[i][j] = cmp::max(dp[i-1][j], dp[i][j-1]);
                }
            }
        }

        // println!("{:?}", dp);
        (l1 + l2) as i32 - 2 * dp[l1][l2]
    }
}
// @lc code=end

