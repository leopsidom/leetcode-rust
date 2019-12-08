/*
 * @lc app=leetcode id=76 lang=rust
 *
 * [76] Minimum Window Substring
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut t_cnt: HashMap<char, i32> = HashMap::new();
        let mut s_cnt: HashMap<char, i32> = HashMap::new();

        let s_chars = s.chars().collect::<Vec<char>>();

        for ch in t.chars() {
            *t_cnt.entry(ch).or_insert(0) += 1;
        }

        let mut n = t_cnt.len();
        let mut left = 0;
        let mut right = 0;
        let mut ans = s.clone();
        let mut has_window = false;

        while right < s_chars.len() {

            let counter = s_cnt.entry(s_chars[right]).or_insert(0);
            *counter += 1;
            if counter == t_cnt.get(&s_chars[right]).unwrap_or(&0) {
                n -= 1;
            }

            while n == 0 {
                has_window = true;
                let ch = s_chars[left];

                let count = s_cnt.get_mut(&ch).unwrap();
                if t_cnt.contains_key(&ch) && count == t_cnt.get(&ch).unwrap() {
                    if right - left < ans.len() {
                        ans = s_chars[left..right+1].iter().collect::<String>();
                    }
                    n += 1;
                }
                
                *count -= 1;
                left += 1;
            }
            right += 1;
        }

        if !has_window {
            "".to_owned()
        } else {
            ans
        }

    }
}
// @lc code=end

