/*
 * @lc app=leetcode id=4 lang=rust
 *
 * [4] Median of Two Sorted Arrays
 */

// @lc code=start
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let l1 = nums1.len();
        let l2 = nums2.len();

        let mid = (l1 + l2 - 1) / 2;
        let odd = (l1 + l2) % 2 == 1;

        let mut i = 0;
        let mut j = 0;

        while i + j < mid {
            if (i < l1) {
                if (j < l2) {
                    if (nums1[i] < nums2[j]) {
                        i += 1;
                    } else {
                        j += 1;
                    }
                } else {
                    i += 1;
                }
            } else {
                j += 1;
            }
        }

        let mut median: f64;

        if (odd) {
            let (nextmin, i, j) = Self::next_min(&nums1, &nums2, i, j);
            nextmin as f64
        } else {
            let (nextmin1, i, j) = Self::next_min(&nums1, &nums2, i, j);
            let (nextmin2, i, j) = Self::next_min(&nums1, &nums2, i, j);
            (nextmin1 + nextmin2) as f64 / 2 as f64
        }

    }

    pub fn next_min(nums1: &Vec<i32>, nums2: &Vec<i32>, i: usize, j: usize) -> (i32, usize, usize) {

        let l1 = nums1.len();
        let l2 = nums2.len();

        if (i < l1) {
            if (j < l2) {
                if (nums1[i] < nums2[j]) {
                    (nums1[i], i+1, j)
                } else {
                    (nums2[j], i, j+1)
                }
            } else {
                (nums1[i], i+1, j)
            }
        } else {
            (nums2[j], i, j+1)
        }
    }
}
// @lc code=end

