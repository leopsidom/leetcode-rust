/*
 * @lc app=leetcode id=566 lang=rust
 *
 * [566] Reshape the Matrix
 */

// @lc code=start
impl Solution {
    pub fn matrix_reshape(nums: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let R = nums.len();
        if R == 0 { return vec![]; }
        let C = nums[0].len();

        if R * C != (r * c) as usize { return nums; }

        let mut ans = vec![];
        let mut row = vec![];

        for i in 0..R {
            for j in 0..C {
                row.push(nums[i][j]);
                if row.len() == c as usize {
                    ans.push(row);
                    row = vec![];
                }
            }
        }

        ans
    }
}
// @lc code=end

