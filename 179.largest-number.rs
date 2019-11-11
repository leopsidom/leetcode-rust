/*
 * @lc app=leetcode id=179 lang=rust
 *
 * [179] Largest Number
 */

// @lc code=start
use std::cmp;
use std::mem;
use std::char;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut num_str = vec![];
        
        for mut num in nums {
            let mut v_num = vec![];

            if num == 0 {
                v_num.push(0);
            } else {
                while num > 0 {
                    v_num.push(num % 10);
                    num /= 10;
                }
            }
            v_num.reverse();

            num_str.push(v_num);
        }

        fn compare<'a>(a: &'a [i32], b: &'a [i32]) -> cmp::Ordering {
            let na = a.len();
            let nb = b.len();

            let n = cmp::min(na, nb);

            for i in 0..n {
                if a[i] < b[i] {
                    return cmp::Ordering::Greater;
                } else if a[i] > b[i] {
                    return cmp::Ordering::Less;
                }
            }

            if na == nb {
                cmp::Ordering::Equal
            } else if na > nb {
                compare(&a[n..], &b[..])
            } else {
                compare(&a[..], &b[n..])
            }
        }

        num_str.sort_by(|a, b| compare(&a[..], &b[..]));

        let mut ans = String::new();

        for v in num_str {
            for d in v {
                ans.push(char::from_digit(d as u32, 10).unwrap());
            }
        }

        let trim_ans = ans.trim_left_matches('0').to_string();

        if trim_ans.len() > 0 { trim_ans } 
        else { "0".to_string() }
    }
}
// @lc code=end

