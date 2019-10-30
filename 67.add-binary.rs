/*
 * @lc app=leetcode id=67 lang=rust
 *
 * [67] Add Binary
 */

// @lc code=start
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let achars: Vec<char> = a.chars().collect();
        let bchars: Vec<char> = b.chars().collect();

        let mut alen = achars.len();
        let mut blen = bchars.len();

        let mut carry = 0;

        let mut i = alen as i32 - 1;
        let mut j = blen as i32 - 1;

        let mut res = String::new();

        while i >= 0 || j >= 0 || carry > 0 {
            let mut achar = 0;
            let mut bchar = 0;
            if i >= 0 {
                achar = achars[i as usize].to_digit(10).unwrap();
                i -= 1;
            }
            if j >= 0 {
                bchar = bchars[j as usize].to_digit(10).unwrap();
                j -= 1;
            }

            let sum = achar + bchar + carry;

            res.push_str(&(sum % 2).to_string());
            carry = sum / 2;
        }

        res.chars().rev().collect()
    }
}
// @lc code=end

