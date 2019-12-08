/*
 * @lc app=leetcode id=97 lang=rust
 *
 * [97] Interleaving String
 */

// @lc code=start
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let chars1 = s1.chars().collect::<Vec<char>>();
        let chars2 = s2.chars().collect::<Vec<char>>();
        let chars3 = s3.chars().collect::<Vec<char>>();

        let n1 = chars1.len();
        let n2 = chars2.len();
        let n3 = chars3.len();

        if n1 + n2 != n3 { return false; }

        let mut dp = vec![vec![false; n2+1]; n1+1];

        for i in 0..n1+1 {
            for j in 0..n2+1 {
                if i == 0 {
                    if j == 0 {
                        dp[i][j] = true;
                    } else {
                        dp[i][j] = dp[i][j-1] && chars2[j-1] == chars3[i+j-1];
                    }
                } else {
                    if j == 0 {
                        dp[i][j] = dp[i-1][j] && chars1[i-1] == chars3[i+j-1];
                    } else {
                        dp[i][j] = (dp[i][j-1] && chars2[j-1] == chars3[i+j-1]) ||
                            (dp[i-1][j] && chars1[i-1] == chars3[i+j-1]);
                    }
                }
            }
        }

        dp[n1][n2]
    }
}
// @lc code=end

