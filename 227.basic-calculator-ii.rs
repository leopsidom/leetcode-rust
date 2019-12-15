/*
 * @lc app=leetcode id=227 lang=rust
 *
 * [227] Basic Calculator II
 */

// @lc code=start
use std::collections::HashSet;

impl Solution {
    pub fn calculate(mut s: String) -> i32 {
        let mut digits = vec![];
        let mut ops = vec![];

        let mut curr = 0;
        s.push('*');

        for ch in s.chars() {
            if ch.is_ascii_digit() {
                curr *= 10;
                curr += ch.to_digit(10).unwrap_or_default() as i32;
            } else if ch != ' ' {
                match ops.pop() {
                    Some('*') => {
                        let prev = digits.pop().unwrap();
                        digits.push(prev * curr);
                    },
                    Some('/') => {
                        let prev = digits.pop().unwrap();
                        digits.push(prev / curr);
                    },
                    Some(op) => {
                        digits.push(curr);
                        ops.push(op);
                    },
                    None => {
                        digits.push(curr);
                    }
                }

                ops.push(ch);
                curr = 0;
            }
        }

        ops.pop();

        // println!("digits: {:?}, ops: {:?}, curr: {}", digits, ops, curr);

        let mut ans = *digits.first().unwrap_or(&0);

        for i in 0..digits.len()-1 {
            match ops[i] {
                '+' => {
                    ans += digits[i+1];
                },
                '-' => {
                    ans -= digits[i+1];
                },
                _ => {}
            }
        }

        ans
    }
}
// @lc code=end

