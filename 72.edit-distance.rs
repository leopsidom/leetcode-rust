/*
 * @lc app=leetcode id=72 lang=rust
 *
 * [72] Edit Distance
 */

// @lc code=start
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let chars1: Vec<char> = word1.chars().collect();
        let chars2: Vec<char> = word2.chars().collect();

        let N1 = chars1.len();
        let N2 = chars2.len();

        let mut dp = vec![vec![0; N2+1]; N1+1];

        for i in 0..N1+1 {
            for j in 0..N2+1 {
                if i == 0 {
                    dp[i][j] = j;
                } else if j == 0 {
                    dp[i][j] = i;
                } else if chars1[i-1] == chars2[j-1] {
                    dp[i][j] = dp[i-1][j-1];
                } else {
                    dp[i][j] = *[
                        dp[i][j-1], 
                        dp[i-1][j], 
                        dp[i-1][j-1]
                    ].iter().min().unwrap() + 1;
                }
            }
        }

        dp[N1][N2] as i32
    }
}
// @lc code=end

