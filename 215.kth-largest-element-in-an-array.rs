/*
 * @lc app=leetcode id=215 lang=rust
 *
 * [215] Kth Largest Element in an Array
 */

// @lc code=start
use std::collections::BinaryHeap;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let N = nums.len();
        let mut heap = BinaryHeap::new();

        for i in 0..N {
            if i < k as usize {
                heap.push(-nums[i]);
            } else {
                if (heap.peek().unwrap() > &-nums[i]) {
                    heap.pop();
                    heap.push(-nums[i]);
                }
            }
        }

        -heap.peek().unwrap()
    }
}
// @lc code=end

