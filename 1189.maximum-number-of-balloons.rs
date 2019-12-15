/*
 * @lc app=leetcode id=1189 lang=rust
 *
 * [1189] Maximum Number of Balloons
 */

// @lc code=start
use std::collections::HashMap;
use std::cmp;

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut ballon_count = HashMap::new();
        for ch in "balon".chars() {
            ballon_count.insert(ch, 0);
        }

        for ch in text.chars() {
            match ballon_count.get_mut(&ch) {
                Some(val) => *val += 1,
                None => ()
            }
        }

        let mut ans = text.len();

        for ch in "balon".chars() {

            let cnt = *ballon_count.get(&ch).unwrap();
            match ch {
                'l' | 'o' => {
                    ans = cmp::min(ans, cnt / 2);
                },
                'b' | 'a' | 'n' => {
                    ans = cmp::min(ans, cnt);
                },
                _ => ()
            }
        }
        
        ans as i32
    }
}
// @lc code=end

