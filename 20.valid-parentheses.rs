/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 */

// @lc code=start
use std::collections::LinkedList;
use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = LinkedList::new();
        let mut map = HashMap::new();
        map.insert(')', '(');
        map.insert('}', '{');
        map.insert(']', '[');

        for c in s.chars() {
            if (c == '(' || c == '{' || c == '[') {
                stack.push_back(c);
            }
            else if (stack.len() > 0) {
                let prev = stack.pop_back().unwrap();
                match map.get(&c) {
                    Some(mapTo) => {
                        if (mapTo != &prev) {
                            return false;
                        }
                    },
                    None => return false
                }
            } else {
                return false;
            }
        }

        return stack.len() == 0;
    }
}
// @lc code=end

