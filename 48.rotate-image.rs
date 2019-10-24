/*
 * @lc app=leetcode id=48 lang=rust
 *
 * [48] Rotate Image
 */

// @lc code=start
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let N = matrix.len();

        for i in 0..N {
            for j in (i+1)..N {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }

        for i in 0..N {
            for j in 0..(N/2) {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[i][N-j-1];
                matrix[i][N-j-1] = tmp;
            }
        }
    }
}
// @lc code=end

