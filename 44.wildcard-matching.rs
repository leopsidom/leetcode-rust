/*
 * @lc app=leetcode id=44 lang=rust
 *
 * [44] Wildcard Matching
 */

// @lc code=start
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let schars: Vec<char> = s.chars().collect();
        let pchars: Vec<char> = p.chars().collect();

        let s_len = schars.len();
        let p_len = pchars.len();

        let mut dp = vec![vec![false; s_len + 1]; p_len + 1];

        for i in 0..p_len+1 {
            for j in 0..s_len+1 {
                if i == 0 {
                    dp[i][j] = j == 0;
                } else if j == 0 {
                    dp[i][j] = dp[i-1][j] && pchars[i-1] == '*';
                } else {
                    if pchars[i-1] == '?' || pchars[i-1] == schars[j-1] {
                        dp[i][j] = dp[i-1][j-1];
                    } else if pchars[i-1] == '*' {
                        for k in 0..j+1 {
                            if dp[i-1][k] {
                                dp[i][j] = true;
                                break;
                            }
                        }
                    }
                }
            }
        }

        // println!("{:?}", dp);
        dp[p_len][s_len]
    }

}
// @lc code=end

