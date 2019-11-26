/*
 * @lc app=leetcode id=31 lang=rust
 *
 * [31] Next Permutation
 */

// @lc code=start
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let N = nums.len();

        let mut i = N - 1;
        while i > 0 && nums[i] <= nums[i-1] {
            i -= 1;
        }
        
        if i == 0 { 
            nums.sort();
            return;
        }

        let left = i - 1;

        let mut right = left + 1;

        while right < N && nums[left] < nums[right] {
            right += 1;
        }

        nums.swap(left, right - 1);

        nums[left+1..].sort()
    }
}
// @lc code=end

