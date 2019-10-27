/*
 * @lc app=leetcode id=32 lang=rust
 *
 * [32] Longest Valid Parentheses
 */

// @lc code=start
use std::cmp;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut chars: Vec<char> = s.chars().collect();

        let left2right = Self::longest_valid_parentheses_one_direction(&chars);
        
        let mut charsMirror = vec![];
        chars.reverse();
        for ch in chars {
            if ch == '(' {
                charsMirror.push(')');
            } else {
                charsMirror.push('(');
            }
        }

        let right2left = Self::longest_valid_parentheses_one_direction(&charsMirror);
        
        cmp::max(left2right, right2left)
    }

    pub fn longest_valid_parentheses_one_direction(chars: &Vec<char>) -> i32 {
        let N = chars.len();
        if N <= 1 { return 0; }

        let mut balance = 0;
        let mut ans = 0;
        let mut start: i32 = -1;

        for i in 0..N {
            if chars[i] == '(' {
                balance += 1;
            } else {
                balance -= 1;
            }

            if balance < 0 {
                start = i as i32;
                balance = 0; 
            } else if balance == 0 {
                ans = cmp::max(ans, i as i32 - start);
            }
        }

        ans
    }
}
// @lc code=end

