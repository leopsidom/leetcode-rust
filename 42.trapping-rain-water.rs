/*
 * @lc app=leetcode id=42 lang=rust
 *
 * [42] Trapping Rain Water
 */

// @lc code=start
use std::cmp;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        // Self::trap_two_way(height)
        Self::trap_two_pointer(height)
    }

    pub fn trap_two_way(height: Vec<i32>) -> i32 {
        let N = height.len();
        if N <= 2 { return 0; }

        let mut max_from_left = vec![0];
        let mut max_from_right = vec![0];

        for i in 1..N {
            max_from_left.push(
                cmp::max(*max_from_left.last().unwrap(), height[i-1])
            );    
        }

        for i in (0..N-1).rev() {
            max_from_right.push(
                cmp::max(*max_from_right.last().unwrap(), height[i+1])
            );
        }

        max_from_right.reverse();

        let mut ans = 0;
        for i in 1..N-1 {
            ans += cmp::max(
                cmp::min(max_from_left[i], max_from_right[i]) - height[i],
                0
            )
        }

        ans
    }

    pub fn trap_two_pointer(height: Vec<i32>) -> i32 {
        if height.len() <= 2 { return 0; }
        let mut left = 0;
        let mut right = height.len() - 1;

        let mut ans = 0;

        let mut maxleft = 0;
        let mut maxright = 0;

        while left <= right {
            if height[left] <= height[right] {
                if maxleft < height[left] {
                    maxleft = height[left];
                } else {
                    ans += maxleft - height[left];
                }

                left += 1;
            } else {
                if maxright < height[right] {
                    maxright = height[right];
                } else {
                    ans += maxright - height[right];
                }

                right -= 1;
            }
        }

        ans
    }
}
// @lc code=end

