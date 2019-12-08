/*
 * @lc app=leetcode id=57 lang=rust
 *
 * [57] Insert Interval
 */

// @lc code=start
use std::mem;
use std::cmp;

impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let N = intervals.len();
        if N == 0 { return vec![new_interval]; }

        let mut ans = vec![];
        let mut i = 0;

        while i < N && intervals[i][1] < new_interval[0] {
            ans.push(mem::replace(&mut intervals[i], vec![]));
            i += 1;
        }

        if i < N {
            let left = cmp::min(intervals[i][0], new_interval[0]);
            let mut right = new_interval[1];

            while i < N && intervals[i][0] <= new_interval[1] {
                if intervals[i][1] > right {
                    right = intervals[i][1];
                }
                i += 1;
            }

            ans.push(vec![left, right]);
        } else {
            ans.push(new_interval);
        }

        while i < N {
            ans.push(mem::replace(&mut intervals[i], vec![]));
            i += 1;
        }
        
        ans
    }
}
// @lc code=end

