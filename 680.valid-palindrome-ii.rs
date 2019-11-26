/*
 * @lc app=leetcode id=680 lang=rust
 *
 * [680] Valid Palindrome II
 */

// @lc code=start
impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let chars = s.chars().collect::<Vec<char>>();
        let N = chars.len();

        if N <= 1 { return true; }
        let mut i = 0;

        while i < N / 2 {
            if chars[i] != chars[N - i - 1] {
                return Self::is_palindrome(&chars[i..N-i-1]) || Self::is_palindrome(&chars[i+1..N-i]);
            }
            i += 1;
        }

        return true;
    }

    pub fn is_palindrome(chars: &[char]) -> bool {
        let N = chars.len();

        if N <= 1 { return true; }

        let mut i = 0;

        while i < N / 2 {
            if chars[i] != chars[N - i - 1] {
                return false;
            }
            i += 1;
        }

        return true;
    }
}
// @lc code=end

