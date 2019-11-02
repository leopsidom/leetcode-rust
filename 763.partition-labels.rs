/*
 * @lc app=leetcode id=763 lang=rust
 *
 * [763] Partition Labels
 */

// @lc code=start
use std::cmp;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let chars = s.chars().collect::<Vec<char>>();

        let N = chars.len();

        let mut ranges = vec![vec![-1, -1]; 26];

        for i in 0..N {
            let ord = chars[i] as usize - 'a' as usize;
            if ranges[ord][0] == -1 {
                ranges[ord][0] = i as i32;
                ranges[ord][1] = i as i32;
            } else {
                ranges[ord][1] = i as i32;
            }
        }

        ranges.sort_by_key(|k| k[0]);

        let mut curr_end = -1;
        let mut ans = vec![];

        for v in ranges {
            if v[0] >= 0 {
                if curr_end < v[0] {
                    ans.push(v[0]);
                }
                curr_end = cmp::max(curr_end, v[1]);
            }
        }

        if ans[ans.len() - 1] < N as i32 {
            ans.push(N as i32);
        }

        let mut ans1 = vec![];

        for i in 1..ans.len() {
            ans1.push(ans[i] - ans[i-1]);
        }

        ans1
    }
}
// @lc code=end

