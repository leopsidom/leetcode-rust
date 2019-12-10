/*
 * @lc app=leetcode id=221 lang=rust
 *
 * [221] Maximal Square
 */

// @lc code=start
use std::cmp;

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let R = matrix.len();
        if R == 0 { return 0; }
        let C = matrix[0].len();

        let mut rows = 0;
        let mut cols = vec![0; C];
        let mut prev_row_area = vec![0; C];
        let mut ans = 0;

        for i in 0..R {
            let mut next_row_area = vec![];
            for j in 0..C {
                if j == 0 {
                    rows = 0;
                }

                if matrix[i][j] == '1' {
                    if j == 0 {
                        next_row_area.push(1);
                    } else {
                        next_row_area.push(
                            *[
                                rows, cols[j], prev_row_area[j-1]
                            ].iter().min().unwrap() + 1
                        );
                    }
                    ans = cmp::max(ans, (next_row_area[j] as i32).pow(2));
                    rows += 1;
                    cols[j] += 1;
                } else {
                    rows = 0;
                    cols[j] = 0;
                    next_row_area.push(0);
                }
            }
            prev_row_area = next_row_area;
        }

        ans
    }
}
// @lc code=end

