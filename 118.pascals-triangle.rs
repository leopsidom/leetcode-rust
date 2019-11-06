/*
 * @lc app=leetcode id=118 lang=rust
 *
 * [118] Pascal's Triangle
 */

// @lc code=start
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut curr = vec![1];

        for i in 0..num_rows {
            let mut next = vec![1];
            for i in 0..curr.len()-1 {
                next.push(curr[i] + curr[i+1]);
            }
            next.push(1);
            ans.push(curr);
            curr = next;
        }

        ans
    }
}
// @lc code=end

