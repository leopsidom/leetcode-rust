/*
 * @lc app=leetcode id=260 lang=rust
 *
 * [260] Single Number III
 */

// @lc code=start
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut xor_tot = 0;
        for num in nums.iter() {
            xor_tot ^= num;
        }

        xor_tot &= - xor_tot;

        let mut a = 0;
        let mut b = 0;

        for num in nums {
            if xor_tot & num == 0 {
                a ^= num;
            } else {
                b ^= num;
            }
        }

        vec![a, b]
    }
}
// @lc code=end

