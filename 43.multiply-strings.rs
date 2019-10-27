/*
 * @lc app=leetcode id=43 lang=rust
 *
 * [43] Multiply Strings
 */

// @lc code=start
use std::char;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let num1chars: Vec<char> = num1.chars().collect();
        let num2chars: Vec<char> = num2.chars().collect();

        let N1 = num1.len();
        let N2 = num2.len();

        let mut ans: Vec<u32> = vec![0; N1 + N2];

        for i in 0..N1 {
            for j in 0..N2 {
                let x1 = num1chars[i].to_digit(10).unwrap_or(0);
                let x2 = num2chars[j].to_digit(10).unwrap_or(0);
                Self::add_num(&mut ans, x1 * x2, N1 - i + N2 - j - 2);
            }
        }

        let mut s = String::new();

        for num in ans {
            s.push(char::from_digit(num, 10).unwrap_or('0'));
        }

        let ans = s.trim_left_matches('0').to_string();
        if ans == "" { "0".to_string() } else { ans }
    }

    pub fn add_num(ans: &mut Vec<u32>, mut num: u32, shift: usize) {
        let N = ans.len();
        let mut idx = N - shift - 1;
        let mut carry = 0;

        while (idx >= 0 && num > 0) || carry > 0 {
            let sum = ans[idx] + num % 10 + carry;
            ans[idx] = sum % 10;
            carry = sum / 10;
            num /= 10;
            idx -= 1;
        }
    }
}
// @lc code=end

