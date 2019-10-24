/*
 * @lc app=leetcode id=766 lang=rust
 *
 * [766] Toeplitz Matrix
 */

// @lc code=start
impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let R = matrix.len();
        if R == 0 {
            return true;
        }

        let C = matrix[0].len();
        let mut vals: Vec<Option<i32>> = vec![None; R + C - 1];

        for i in 0..R {
            for j in 0..C {
                let d = i - j + C - 1;
                if let Some(val) = vals[d] {
                    if val != matrix[i][j] {
                        return false;
                    }
                } else {
                    vals[d] = Some(matrix[i][j]);
                }
            }
        }

        return true;
    }
}
// @lc code=end

