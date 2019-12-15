/*
 * @lc app=leetcode id=1232 lang=rust
 *
 * [1232] Check If It Is a Straight Line
 */

// @lc code=start
impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        if (coordinates.len() <= 2) {
            return true;
        }
        
        let N = coordinates.len();
        
        let p1 = &coordinates[0];
        let p2 = &coordinates[1];
        
        for i in 2..N {
            let p3 = &coordinates[i];
            if !Self::same_line_with(p1, p2, p3) {
                return false;
            }
        }
        
        return true;
    }

    pub fn same_line_with(p1: &Vec<i32>, p2: &Vec<i32>, p3: &Vec<i32>) -> bool {
        (p2[1] - p1[1]) * (p3[0] - p1[0]) == (p3[1] - p1[1]) * (p2[0] - p1[0])
    }
}
// @lc code=end

