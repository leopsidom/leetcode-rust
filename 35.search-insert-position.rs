/*
 * @lc app=leetcode id=35 lang=rust
 *
 * [35] Search Insert Position
 */

// @lc code=start
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut lo = 0;
        let mut hi = nums.len();

        while lo < hi {
            let mid = (lo + hi) / 2;
            let val = *nums.get(mid).unwrap();

            if (val == target) {
                return mid as i32;
            } else if (val < target) {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }

        lo as i32
    }
}
// @lc code=end

