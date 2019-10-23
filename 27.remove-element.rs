/*
 * @lc app=leetcode id=27 lang=rust
 *
 * [27] Remove Element
 */

// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.push(val);

        let mut i = 0;
        let mut j = nums.len() - 1;

        while i < j {
            let val_i = nums.get(i).unwrap();
            if (*val_i == val) {
                let mut val_j = nums.get(j).unwrap(); 
                while j > i && *val_j == val {
                    j -= 1;
                    val_j = nums.get(j).unwrap(); 
                }
                if (i == j) {
                    return i as i32;
                }
                nums.swap(i, j);
            }
            i += 1;
        }
        return i as i32;
    }
}
// @lc code=end

