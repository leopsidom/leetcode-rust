/*
 * @lc app=leetcode id=85 lang=rust
 *
 * [85] Maximal Rectangle
 */

// @lc code=start
use std::cmp;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let R = matrix.len();
        if R == 0 { return 0; }
        let C = matrix[0].len();

        let mut cols = vec![0; C+1];
        let mut ans = 0;

        for i in 0..R {
            for j in 0..C {
                if matrix[i][j] == '1' {
                    cols[j+1] += 1;
                } else {
                    cols[j+1] = 0;
                }
            }

            ans = cmp::max(ans, Self::largest_rectangle_area(&cols));
        }

        ans
    }

    pub fn largest_rectangle_area(heights: &Vec<i32>) -> i32 {
        let N = heights.len();

        let mut less_from_left = vec![0; N];
        let mut less_from_right = vec![0; N];

        for i in 0..N {
            let mut p = i as i32 - 1;

            while p >= 0 && heights[p as usize] >= heights[i] {
                p = less_from_left[p as usize];
            }

            less_from_left[i] = p;
        }

        for i in (0..N).rev() {
            let mut p = i as i32 + 1;

            while p < N as i32 && heights[p as usize] >= heights[i] {
                p = less_from_right[p as usize];
            }

            less_from_right[i] = p;
        }

        let mut ans = 0;
        for i in 0..N {
            ans = cmp::max(ans, heights[i] * (less_from_right[i] - less_from_left[i] - 1));
        }

        ans
    }
}
// @lc code=end

