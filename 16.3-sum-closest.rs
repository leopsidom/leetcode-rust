/*
 * @lc app=leetcode id=16 lang=rust
 *
 * [16] 3Sum Closest
 */

// @lc code=start
impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        let N = nums.len();

        nums.sort();

        let mut ans = 0;
        for i in 0..3 {
            ans += nums[i];
        }
        
        let mut i = 0;
        let mut j = 1;
        let mut k = N - 1;

        while i < N - 2 {
            j = i + 1;
            k = N - 1;

            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                if ((sum - target).abs() < (ans - target).abs()) {
                    ans = sum;
                }
                if (sum < target) {
                    j += 1;
                } else if (sum > target) {
                    k -= 1;
                } else {
                    return sum;
                }
            }

            while i < N - 2 && nums[i] == nums[i+1] {
                i += 1;
            }

            i += 1;
        }

        ans
    }
}
// @lc code=end

