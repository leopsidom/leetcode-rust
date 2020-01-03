/*
 * @lc app=leetcode id=581 lang=rust
 *
 * [581] Shortest Unsorted Continuous Subarray
 */

// @lc code=start
impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let N = nums.len();
        if N <= 1 { return 0; }

        let mut start = N - 1;
        let mut currmin = nums[N-1];

        for i in (0..N-1).rev() {
            if nums[i] > currmin {
                start = i;
            } else {
                currmin = nums[i];
            }
        }

        let mut end = 0;
        let mut currmax = nums[0];

        for i in 1..N {
            if nums[i] < currmax {
                end = i;
            } else {
                currmax = nums[i];
            }
        }

        if end > start { end as i32 - start as i32 + 1} else { 0 }
    }
}
// @lc code=end

