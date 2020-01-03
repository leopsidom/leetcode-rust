/*
 * @lc app=leetcode id=238 lang=rust
 *
 * [238] Product of Array Except Self
 */

// @lc code=start
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut prod = 1;
        let mut zeros = 0;

        for num in nums.iter() {
            if *num == 0 {
                zeros += 1;
            } else {
                prod *= num;
            }
        }

        let mut output = vec![0; nums.len()];
        if zeros >= 2 {
            return output;
        }

        if zeros == 1 {
            for i in 0..nums.len() {
                if nums[i] == 0 {
                    output[i] = prod;
                }
            }
            return output;
        }

        for i in 0..nums.len() {
            output[i] = prod / nums[i];
        }
        return output;
    }
}
// @lc code=end

