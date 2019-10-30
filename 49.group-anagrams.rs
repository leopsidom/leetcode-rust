/*
 * @lc app=leetcode id=49 lang=rust
 *
 * [49] Group Anagrams
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<Vec<u32>, Vec<String>> = HashMap::new();

        for s in strs {
            let mut cnt = vec![0; 26];
            for ch in s.chars() {
                cnt[ch as usize - 'a' as usize] += 1;
            }
            let val = map.entry(cnt).or_insert(vec![]);
            (*val).push(s);
        }

        let mut ans = vec![];

        for vec in map.values() {
            ans.push(vec.clone());
        }
        ans

    }
}
// @lc code=end

