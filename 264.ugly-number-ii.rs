/*
 * @lc app=leetcode id=264 lang=rust
 *
 * [264] Ugly Number II
 */

// @lc code=start
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        if n <= 0 { return -1; }
        let mut ugly_numbers = vec![1; n as usize];
        let mut i2 = 0;
        let mut i3 = 0;
        let mut i5 = 0;

        for i in 1..n as usize {
            ugly_numbers[i] = *[
                ugly_numbers[i2] * 2,
                ugly_numbers[i3] * 3,
                ugly_numbers[i5] * 5
                ].iter().min().unwrap();
            
            if ugly_numbers[i] == ugly_numbers[i2] * 2 { i2 += 1; }
            if ugly_numbers[i] == ugly_numbers[i3] * 3 { i3 += 1; }
            if ugly_numbers[i] == ugly_numbers[i5] * 5 { i5 += 1; }
        }

        ugly_numbers[n as usize - 1]
    }
}
// @lc code=end

