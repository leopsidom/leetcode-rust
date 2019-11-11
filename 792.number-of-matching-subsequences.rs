/*
 * @lc app=leetcode id=792 lang=rust
 *
 * [792] Number of Matching Subsequences
 */

// @lc code=start
impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        fn is_subseq(word1: &str, word2: &str) -> bool {
            let mut iter1 = word1.chars();
            let mut iter2 = word2.chars();

            while let Some(ch1) = iter1.next() {
                loop { 
                    if let Some(ch2) = iter2.next() {
                        if ch1 == ch2 { break; }
                    } else {
                        return false;
                    }
                }
            }

            return true;
        }

        let mut ans = 0;
        for word2 in words {
            if is_subseq(&word2, &s) {
                ans += 1;
            }
        }

        ans
    }
}
// @lc code=end

