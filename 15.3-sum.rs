/*
 * @lc app=leetcode id=15 lang=rust
 *
 * [15] 3Sum
 */

// @lc code=start
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();

        let n = nums.len();
        let mut ans: Vec<Vec<i32>> = vec![];
        if (n < 3) {
            return ans;
        }

        let mut i = 0;
        let mut j = 1;
        let mut k = n-1;

        while i <= n - 2 {
            j = i + 1;
            k = n - 1;

            while j < k {
                if nums[i] + nums[j] + nums[k] < 0 {
                    j += 1;
                } else if nums[i] + nums[j] + nums[k] > 0 {
                    k -= 1;
                } else {
                    ans.push(vec![nums[i], nums[j], nums[k]]);
                    while j < k && nums[j] == nums[j+1] {
                        j += 1;
                    }
                    j += 1;
                    while j < k && nums[k-1] == nums[k] {
                        k -= 1;
                    }
                    k -= 1;
                }

            }

            while i <= n - 2 && nums[i] == nums[i+1] {
                i += 1;
            }
            i += 1;
        }

        ans
    }
}
// @lc code=end

