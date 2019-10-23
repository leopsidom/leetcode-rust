/*
 * @lc app=leetcode id=66 lang=rust
 *
 * [66] Plus One
 */

// @lc code=start
impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        digits.reverse();

        let mut ans: Vec<i32> = Vec::new();
        let mut carry = 1;

        for d in digits {
            let sum = d + carry;
            ans.push(sum % 10);
            carry = sum / 10;
        }

        if carry != 0 {
            ans.push(carry);
        }

        ans.reverse();
        ans
    }
}
// @lc code=end

