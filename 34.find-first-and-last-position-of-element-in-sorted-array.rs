/*
 * @lc app=leetcode id=34 lang=rust
 *
 * [34] Find First and Last Position of Element in Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let N = nums.len();

        if (N == 0) {
            return vec![-1, -1];
        }

        let mut lo = 0;
        let mut hi = nums.len() - 1;

        while lo < hi {
            let mid = (lo + hi) / 2;

            if (nums[mid] < target) {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }

        if (nums[lo] != target) {
            return vec![-1, -1];
        }

        let mut l1 = 0;
        let mut h1 = lo;

        while l1 < h1 {
            let mid = (l1 + h1) / 2;

            if (nums[mid] < target) {
                l1 = mid + 1;
            } else {
                h1 = mid;
            }
        }

        let mut l2 = lo;
        let mut h2 = N - 1;

        while l2 < h2 {
            let mid = (l2 + h2 + 1) / 2;

            if (nums[mid] == target) {
                l2 = mid;
            } else {
                h2 = mid - 1;
            }
        }

        return vec![l1 as i32, h2 as i32];
    }
}
// @lc code=end

