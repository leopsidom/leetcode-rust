/*
 * @lc app=leetcode id=33 lang=rust
 *
 * [33] Search in Rotated Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let N = nums.len();

        if (N == 0) {
            return -1;
        } else if (N == 1) {
            if (nums[0] == target) {
                return 0;
            } else {
                return -1;
            }
        }

        let mut lo = 0;
        let mut hi = N - 1;

        while lo < hi {
            let mid = (lo + hi) / 2;

            if (target <= nums[N-1]) {
                if (nums[mid] > nums[N-1]) {
                    lo = mid + 1;
                } else {
                    if (nums[mid] < target) {
                        lo = mid + 1;
                    } else if (nums[mid] > target) {
                        hi = mid;
                    } else {
                        return mid as i32;
                    }
                }
            } else {
                if (nums[mid] <= nums[N-1]) {
                    hi = mid;
                } else {
                    if (nums[mid] < target) {
                        lo = mid + 1;
                    } else if (nums[mid] > target) {
                        hi = mid;
                    } else {
                        return mid as i32;
                    }
                }
            }

        }

        if (nums[lo] == target) {
            return lo as i32;
        } else {
            return -1;
        }
        
    }
}
// @lc code=end

